use anyhow::{Context, Result};
use aya::programs::TracePoint;
use aya::Bpf;
use clap::Parser;
use log::info;
use std::path::Path;
use tokio::signal;

mod stats;
use stats::SocketDropStats;

#[derive(Debug, Parser)]
#[command(name = "kfree_skb")]
#[command(about = "eBPF tracer for socket drop reasons via kfree_skb tracepoint", long_about = None)]
#[command(version)]
struct Args {
    /// Print statistics and exit (don't run continuously)
    #[arg(short, long)]
    stats: bool,

    /// Print verbose logging from eBPF program
    #[arg(short, long)]
    verbose: bool,

    /// Path to the eBPF object file (auto-detected if not provided)
    #[arg(long)]
    ebpf: Option<String>,

    /// Duration to collect statistics for (in seconds, for --stats mode)
    #[arg(short, long, default_value = "5")]
    duration: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Setup logging
    setup_logging(args.verbose)?;

    info!("Starting kfree_skb tracer");

    // Load the eBPF program
    let ebpf_path = find_ebpf_binary(&args.ebpf)?;
    info!("Loading eBPF program from: {}", ebpf_path);

    let mut bpf = load_ebpf_program(&ebpf_path)?;

    // Attach the tracepoint
    attach_tracepoint(&mut bpf)?;

    info!("Attached to tracepoint skb/kfree_skb");

    if args.stats {
        // Collect statistics for a period of time
        collect_and_print_statistics(&mut bpf, args.duration).await?;
    } else {
        // Run continuously until interrupted
        run_tracer(&bpf).await?;
    }

    info!("Shutting down cleanly");
    Ok(())
}

fn setup_logging(verbose: bool) -> Result<()> {
    let level = if verbose {
        log::LevelFilter::Info
    } else {
        log::LevelFilter::Warn
    };

    env_logger::Builder::from_default_env()
        .filter_level(level)
        .format_timestamp_millis()
        .try_init()
        .context("Failed to initialize logger")?;

    Ok(())
}

fn find_ebpf_binary(explicit_path: &Option<String>) -> Result<String> {
    if let Some(path) = explicit_path {
        return Ok(path.clone());
    }

    // Check common locations
    let possible_paths = vec![
        "target/bpf/x86_64-unknown-linux-bpf/release/kfree_skb.o",
        "target/bpf/x86_64-unknown-linux-gnu/release/kfree_skb.o",
        "target/bpf/x86_64-unknown-none/release/kfree_skb.o",
        "ebpf/target/bpf/x86_64-unknown-linux-bpf/release/kfree_skb.o",
        "./kfree_skb.bpf.o",
    ];

    for path in &possible_paths {
        if Path::new(path).exists() {
            info!("Found eBPF object at: {}", path);
            return Ok(path.to_string());
        }
    }

    Err(anyhow::anyhow!(
        "Could not find compiled eBPF object. Checked:\n  {}\n\nPlease build with: cargo build --release",
        possible_paths.join("\n  ")
    ))
}

fn load_ebpf_program(path: &str) -> Result<Bpf> {
    let bytes = std::fs::read(path)
        .with_context(|| format!("Failed to read eBPF object file at {}", path))?;

    let bpf = Bpf::load(&bytes).context("Failed to load eBPF program")?;

    info!("Successfully loaded eBPF program");
    Ok(bpf)
}

fn attach_tracepoint(bpf: &mut Bpf) -> Result<()> {
    let program: &mut TracePoint = bpf
        .program_mut("trace_kfree_skb")
        .context("Failed to find trace_kfree_skb program")?
        .try_into()
        .context("Program is not a tracepoint")?;

    program.load().context("Failed to load trace_kfree_skb")?;
    program
        .attach("skb", "kfree_skb")
        .context("Failed to attach to tracepoint")?;

    Ok(())
}

async fn run_tracer(_bpf: &Bpf) -> Result<()> {
    info!("Running tracer (press Ctrl+C to stop)...");

    // Create a signal handler for Ctrl+C
    let mut sigint = signal::unix::signal(signal::unix::SignalKind::interrupt())
        .context("Failed to create signal handler")?;

    loop {
        tokio::select! {
            _ = sigint.recv() => {
                info!("Received interrupt signal, shutting down...");
                break;
            }
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(10)) => {
                // Just keep running, periodically log that we're alive
                info!("Tracer still running...");
            }
        }
    }

    Ok(())
}

async fn collect_and_print_statistics(bpf: &mut Bpf, duration: u64) -> Result<()> {
    info!(
        "Collecting statistics for {} seconds (waiting for events)...",
        duration
    );
    println!("Collecting socket drop statistics for {} seconds...", duration);
    println!("(Generate network traffic to capture events)\n");

    // Wait for the specified duration
    tokio::time::sleep(tokio::time::Duration::from_secs(duration)).await;

    // Read and display statistics
    match SocketDropStats::from_bpf(bpf) {
        Ok(stats) => {
            println!("\n");
            stats.print();
        }
        Err(e) => {
            // If map doesn't exist yet, show a helpful message
            if e.to_string().contains("Failed to get socket_drop_counts map") {
                println!("\nNo statistics available yet.");
                println!("The BPF map is created when the eBPF program processes its first event.");
                println!("\nTo generate events:");
                println!("  - Use: ping <hostname>");
                println!("  - Or: curl http://example.com");
                println!("  - Or any other network activity");
                println!("\nNote: Some kernels may not have this specific tracepoint enabled.");
                return Err(e);
            }
            return Err(e.context("Failed to read statistics from BPF map"));
        }
    }

    Ok(())
}
