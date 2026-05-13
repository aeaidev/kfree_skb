# Quick Start Guide

Get the kfree_skb eBPF tracer up and running in minutes.

## 1. Prerequisites

```bash
# Check Rust installation
rustc --version  # Should be 1.70+

# If not installed:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# The user-space program is ready to use!
# For the eBPF kernel program, you need the BPF target (optional):
rustup target add x86_64-unknown-linux-bpf
```

## 2. Build User-space Program

The user-space binary is already built! If not, build it with:

```bash
cargo build --release

# Output: ./target/release/kfree_skb (2.3 MB binary)
```

## 3. Build eBPF Program (Optional)

The program can use the original `kfree_skb.bpf.o` compiled with clang, or you can build a Rust version:

```bash
# Option 1: Build Rust eBPF program
cd ebpf
cargo build --target x86_64-unknown-linux-bpf --release

# Option 2: Use original C-compiled version
# Place kfree_skb.bpf.o in the project root or use --ebpf flag

# Option 3: From root with make
make ebpf
```

See [BUILDING_EBPF.md](BUILDING_EBPF.md) for detailed instructions.

## 4. Run

### Option A: Continuous Tracing
```bash
# Terminal 1: Start tracer with original C eBPF program
sudo ./target/release/kfree_skb --verbose

# Terminal 2: Generate traffic
ping -c 10 127.0.0.1

# Back to Terminal 1: Press Ctrl+C to stop
```

### Option B: Statistics Only
```bash
sudo ./target/release/kfree_skb --stats
```

### Option C: Using Make
```bash
# One-liner test with traffic
make test-run

# Continuous tracing
make run

# Just statistics
make stats
```

### Option D: Use Original eBPF with Explicit Path
```bash
sudo ./target/release/kfree_skb --verbose --ebpf ./kfree_skb.bpf.o
```

## Output Examples

### Verbose Output
```
[INFO] Starting kfree_skb tracer
[INFO] Loading eBPF program from: ./kfree_skb.bpf.o
[INFO] Successfully loaded eBPF program
[INFO] Attached to tracepoint skb/kfree_skb
[INFO] Running tracer (press Ctrl+C to stop)...
```

### Statistics Output
```
Socket drop statistics:
- NO_SOCKET: 42
- SOCKET_CLOSE: 15
- SOCKET_FILTER: 0
- SOCKET_RCVBUFF: 0
- SOCKET_BACKLOG: 0
- PACKET_SOCK_ERROR: 0
- TOTAL: 57
```

## Common Commands

| Command | What it does |
|---------|---|
| `cargo build --release` | Build user-space binary only |
| `make ebpf` | Build eBPF kernel program (requires nightly) |
| `cargo test` | Run unit tests |
| `cargo fmt` | Format code |
| `cargo clippy` | Check code quality |
| `make help` | Show all make targets |
| `make run` | Start continuous tracing |
| `make stats` | Show statistics only |
| `make test-run` | Run test with ping |

## Troubleshooting

### "Permission denied"
```bash
# Solution: Run with sudo
sudo ./target/release/kfree_skb --verbose

# Or add BPF capabilities
sudo setcap cap_perfmon,cap_bpf+ep ./target/release/kfree_skb
./target/release/kfree_skb --verbose
```

### "Failed to load eBPF program"
```bash
# Check if you have the original kfree_skb.bpf.o
ls -la kfree_skb.bpf.o

# Check kernel support
cat /sys/kernel/btf/vmlinux  # Should exist

# Use the original C-compiled version
sudo ./target/release/kfree_skb --verbose --ebpf ./kfree_skb.bpf.o
```

### "Could not find compiled eBPF object"
```bash
# Option 1: Use original C version
sudo ./target/release/kfree_skb --verbose --ebpf ./kfree_skb.bpf.o

# Option 2: Build Rust eBPF version (requires x86_64-unknown-linux-bpf target)
rustup target add x86_64-unknown-linux-bpf
cd ebpf
cargo build --target x86_64-unknown-linux-bpf --release
cd ..

# Option 3: Specify path explicitly
sudo ./target/release/kfree_skb --verbose --ebpf ./ebpf/target/x86_64-unknown-linux-bpf/release/kfree_skb.o
```

### Build fails with "target not found"
```bash
# Install the BPF target
rustup target add x86_64-unknown-linux-bpf
cd ebpf
cargo build --target x86_64-unknown-linux-bpf --release
```

## Next Steps

- Check [README.md](README.md) for detailed documentation
- Read [BUILDING_EBPF.md](BUILDING_EBPF.md) for eBPF program build details
- Read [IMPLEMENTATION_GUIDE.md](IMPLEMENTATION_GUIDE.md) for conversion details
- Explore the code: `src/main.rs`, `src/stats.rs`, `ebpf/src/main.rs`
- Run tests: `make test`

## Getting Help

```bash
# Show CLI help
./target/release/kfree_skb --help

# Show make targets
make help

# Build documentation
cargo doc --open
```

## What's Happening Behind the Scenes

1. **User-space Program (Built)**
   - Loads eBPF program from disk
   - Kernel verifies and loads eBPF bytecode
   - Tracepoint attachment configured

2. **eBPF Kernel Program (From original C version or Rust)**
   - Hooks into `kfree_skb` tracepoint
   - Counts socket drop reasons
   - Updates counters in BPF map

3. **Runtime Phase**
   - Kernel triggers eBPF code on every `kfree_skb` event
   - eBPF code updates counters in BPF map
   - User-space can read map at any time

4. **Cleanup Phase**
   - Ctrl+C triggers graceful shutdown
   - BPF program automatically detached
   - Resources cleaned up by kernel

## Performance Tips

- Use `--stats` mode for minimal overhead when just collecting statistics
- `--verbose` adds logging overhead but good for debugging
- Binary is optimized with LTO in release mode
- eBPF program runs in kernel with minimal overhead

Enjoy tracing! 🚀
