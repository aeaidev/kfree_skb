# kfree_skb eBPF Tracer - Rust Edition

A high-performance Rust implementation of an eBPF tracepoint program that hooks into `tracepoint/skb/kfree_skb` to track and analyze socket buffer drop reasons in the Linux kernel.

## Overview

This project has been converted from C and Python to pure Rust, leveraging the `aya` eBPF framework for both kernel-space and user-space components.

### Architecture

- **eBPF Program** (`ebpf/src/main.rs`): Kernel-space tracing program that hooks the `kfree_skb` tracepoint
- **User-space Program** (`src/main.rs`): Loads the eBPF program, manages attachments, and reads statistics
- **Statistics Module** (`src/stats.rs`): Reads and displays socket drop statistics from BPF maps

## Prerequisites

### System Requirements
- Linux kernel with eBPF and BPF tracepoint support (5.8+)
- Root privileges or appropriate capabilities for eBPF operations
- `rustup` with the Rust toolchain

### Rust Setup
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add the BPF target
rustup target add x86_64-unknown-linux-gnu
```

### System Tools (Optional)
- `bpftool` - for manual inspection of BPF programs and maps
- `perf` - for performance analysis

## Building

### Full Build
```bash
cargo build --release
```

This will:
1. Compile the eBPF program in kernel-space (no_std, no_main)
2. Compile the user-space Rust program with all dependencies

### Build Just the eBPF Program
```bash
cd ebpf
cargo build --target bpf --release
```

### Build Just the User-space Program
```bash
cargo build --release --bin kfree_skb
```

## Usage

### Basic Operation (Continuous Tracing)
```bash
sudo ./target/release/kfree_skb --verbose
```

This will:
- Load the compiled eBPF program
- Attach to the `skb/kfree_skb` tracepoint
- Print trace events as they occur
- Run until interrupted with Ctrl+C

### Display Statistics
```bash
sudo ./target/release/kfree_skb --stats
```

This will load the program, run it briefly, collect statistics, and display:
- Count for each socket drop reason
- Total number of drops
- Exit cleanly

### With Verbose Logging
```bash
sudo RUST_LOG=info ./target/release/kfree_skb --verbose
```

## Generating Test Traffic

In a separate terminal, generate network traffic to trigger socket drops:

```bash
ping -c 10 127.0.0.1
```

Or use other network utilities:
```bash
curl http://example.com
netstat -i
ss -s
```

## Socket Drop Reasons Tracked

The program tracks the following socket-related drop reasons:

| Reason | Code | Description |
|--------|------|-------------|
| NO_SOCKET | 2 | No socket associated with SKB |
| SOCKET_CLOSE | 3 | Socket is closed |
| SOCKET_FILTER | 4 | Socket filter rejected the packet |
| SOCKET_RCVBUFF | 5 | Socket receive buffer exceeded |
| SOCKET_BACKLOG | 6 | Socket backlog limit exceeded |
| PACKET_SOCK_ERROR | 11 | Packet socket error |

Other drop reasons are reported as "OTHER" with their numeric code.

## Project Structure

```
warp_kfree_skb/
├── Cargo.toml              # Main workspace configuration
├── build.rs                # Build script for eBPF compilation
├── README.md               # This file
├── .cargo/
│   └── config.toml        # Cargo configuration
├── src/
│   ├── main.rs            # User-space program entry point
│   └── stats.rs           # Statistics reading module
├── ebpf/
│   ├── Cargo.toml         # eBPF crate configuration
│   └── src/
│       └── main.rs        # eBPF kernel-space program
└── target/
    └── bpf/               # Compiled eBPF objects
        └── release/
            └── kfree_skb.o
```

## Key Implementation Details

### eBPF Program
- Uses `aya_bpf` for kernel-space eBPF development
- Implements a `tracepoint` handler for `skb/kfree_skb`
- Maintains a `BPF_MAP_TYPE_ARRAY` for counting socket drops
- Maps kernel drop reason codes to readable names
- Uses `no_std` to minimize kernel footprint

### User-space Program
- Uses `aya` for loading and managing eBPF programs
- Provides CLI interface with `clap`
- Async runtime with `tokio` for signal handling
- Graceful shutdown on Ctrl+C
- Comprehensive error handling with `anyhow`

### Statistics Module
- Safely reads from BPF maps
- Parses raw counter values
- Provides formatted output matching original Python script

## Comparison with Original Implementation

### Advantages of Rust Edition
- **Unified Language**: Single language for both kernel and user-space code
- **Safety**: Memory safety without garbage collection
- **Performance**: No runtime overhead, direct kernel interaction
- **Dependency Management**: Cargo handles all dependencies
- **Type Safety**: Compile-time type checking across boundary
- **Easier Deployment**: Single binary distribution

### Feature Parity
- ✅ Kernel tracepoint attachment
- ✅ Socket drop reason tracking
- ✅ Real-time trace output
- ✅ Statistics aggregation
- ✅ Graceful shutdown
- ✅ Error handling

## Common Issues and Solutions

### "Failed to load eBPF program"
- Ensure you have Linux kernel 5.8+ with BPF support
- Check that `/sys/kernel/btf/vmlinux` exists
- Verify kernel is compiled with `CONFIG_BPF=y`

### "Permission denied"
- Run with `sudo` or ensure user has appropriate BPF capabilities
- To add capabilities: `sudo setcap cap_perfmon,cap_bpf+ep ./target/release/kfree_skb`

### "Cannot find symbol in vmlinux"
- The kernel may not have the required tracepoint
- Check: `cat /sys/kernel/debug/tracing/available_events | grep kfree_skb`

### eBPF Build Fails
- Ensure BPF target is installed: `rustup target add x86_64-unknown-linux-gnu`
- Check LLVM version compatibility with your kernel

## Development

### Running Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

### Building Documentation
```bash
cargo doc --open
```

## Performance Characteristics

- **Kernel Footprint**: ~2KB for eBPF program
- **Overhead**: Minimal - only executes on kfree_skb events
- **Map Memory**: 6 × 8 bytes = 48 bytes for counters
- **Latency**: Sub-microsecond per event

## Extending the Program

### Adding New Drop Reasons
Edit `ebpf/src/main.rs`:
1. Add new constant for kernel code (e.g., `SKB_DROP_REASON_NETWORK_ERROR: u32 = 12`)
2. Increase `SOCKET_DROP_COUNTS` max_entries if needed
3. Add case to match statement with new mapping
4. Update statistics arrays accordingly

### Adding New Metrics
1. Create additional BPF maps in `ebpf/src/main.rs`
2. Update the tracepoint handler to populate them
3. Extend `stats.rs` to read and display the new data

## License

This project maintains the same license as the original C/Python implementation. Check the LICENSE file or original project documentation.

## References

- [Aya eBPF Framework](https://github.com/aya-rs/aya)
- [Linux eBPF Documentation](https://ebpf.io/)
- [BPF and XDP Reference Guide](https://docs.cilium.io/en/v1.9/bpf/)
- [Kernel SKB Drop Reasons](https://www.kernel.org/doc/html/latest/networking/skb_drop_reasons.html)

## Troubleshooting

For more detailed debugging:
```bash
# Check available tracepoints
cat /sys/kernel/debug/tracing/available_events | grep kfree

# View kernel messages
sudo dmesg | tail -20

# Inspect loaded eBPF programs
sudo bpftool prog list

# Check attached tracepoints
sudo bpftool prog show id <prog_id>
```
