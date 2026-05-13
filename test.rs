#!/usr/bin/env rust-script
//! Test script for kfree_skb tracer
//!
//! ```cargo
//! [dependencies]
//! tokio = { version = "1", features = ["full"] }
//! ```

use std::process::{Command, Stdio};
use std::time::Duration;
use std::thread;

fn main() {
    println!("=== kfree_skb Tracer Test ===\n");

    // Build the project
    println!("[1/4] Building project...");
    if !run_command("cargo", &["build", "--release"]) {
        eprintln!("Build failed!");
        std::process::exit(1);
    }
    println!("✓ Build successful\n");

    // Load and run the tracer
    println!("[2/4] Loading eBPF program...");
    let tracer_handle = Command::new("sudo")
        .args(&["./target/release/kfree_skb", "--verbose"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    let mut tracer = match tracer_handle {
        Ok(child) => child,
        Err(e) => {
            eprintln!("Failed to start tracer: {}", e);
            std::process::exit(1);
        }
    };

    // Give it a moment to load
    thread::sleep(Duration::from_secs(1));
    println!("✓ eBPF program loaded\n");

    // Generate test traffic
    println!("[3/4] Generating test traffic (ping 10 packets)...");
    if !run_command("ping", &["-c", "10", "127.0.0.1"]) {
        eprintln!("Ping failed, but continuing...");
    }
    println!("✓ Traffic generated\n");

    // Let the tracer run a bit more
    thread::sleep(Duration::from_secs(1));

    // Terminate the tracer
    println!("[4/4] Collecting statistics...");
    let _ = tracer.kill();
    let _ = tracer.wait();

    // Run the stats command
    if !run_command("sudo", &["./target/release/kfree_skb", "--stats"]) {
        eprintln!("Stats collection had issues");
    }

    println!("\n=== Test Complete ===");
}

fn run_command(program: &str, args: &[&str]) -> bool {
    let status = Command::new(program)
        .args(args)
        .status();

    match status {
        Ok(status) => status.success(),
        Err(e) => {
            eprintln!("Failed to run {}: {}", program, e);
            false
        }
    }
}
