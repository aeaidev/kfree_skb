# Building the eBPF Program

This guide explains how to build the kernel-space eBPF program separately.

## Prerequisites

You need the Rust BPF target installed:

```bash
rustup target add x86_64-unknown-linux-bpf
```

If you're on a system without BPF support in the Rust toolchain, you may need to use a nightly Rust version:

```bash
rustup toolchain install nightly
rustup +nightly target add x86_64-unknown-linux-bpf
```

## Building the eBPF Program

### Option 1: Using the build script in the ebpf directory

```bash
cd ebpf
cargo build --target x86_64-unknown-linux-bpf --release
```

The compiled eBPF object will be at:
```
ebpf/target/x86_64-unknown-linux-bpf/release/kfree_skb.o
```

### Option 2: From the root with make

```bash
make ebpf
```

### Option 3: Manual compilation with cargo

```bash
cargo build -p kfree_skb_ebpf --target x86_64-unknown-linux-bpf --release
```

## Verifying the Build

After building, verify the eBPF object exists:

```bash
file target/bpf/x86_64-unknown-linux-bpf/release/kfree_skb.o
# Should output: ELF 64-bit LSB relocatable, eBPF, version 1 (SYSV), not stripped
```

Or check with llvm-objdump if available:

```bash
llvm-objdump -d target/bpf/x86_64-unknown-linux-bpf/release/kfree_skb.o | head -30
```

## Location Detection

The user-space program will automatically look for the compiled eBPF object in:

1. `target/bpf/x86_64-unknown-linux-bpf/release/kfree_skb.o`
2. `target/bpf/x86_64-unknown-linux-gnu/release/kfree_skb.o`
3. `target/bpf/x86_64-unknown-none/release/kfree_skb.o`
4. `ebpf/target/bpf/x86_64-unknown-linux-bpf/release/kfree_skb.o`
5. `./kfree_skb.bpf.o` (original C-compiled version)

If none of these are found, you can specify the path explicitly:

```bash
sudo ./target/release/kfree_skb --verbose --ebpf ./path/to/kfree_skb.o
```

## Troubleshooting

### "Target not found" error

```bash
# Install the BPF target
rustup target add x86_64-unknown-linux-bpf

# Or use nightly
rustup toolchain install nightly
rustup +nightly target add x86_64-unknown-linux-bpf
rustup +nightly build -p kfree_skb_ebpf --target x86_64-unknown-linux-bpf --release
```

### LLVM version mismatch

If you get LLVM version errors, try using the nightly Rust toolchain which has more recent LLVM:

```bash
rustup toolchain install nightly
cd ebpf
cargo +nightly build --target x86_64-unknown-linux-bpf --release
```

### Permission or file not found

Make sure your kernel supports eBPF and BTF:

```bash
# Check if kernel has BPF support
grep CONFIG_BPF= /boot/config-$(uname -r)

# Check for BTF support
cat /sys/kernel/btf/vmlinux
```

## Using the Original C-compiled Version

If you have the original `kfree_skb.bpf.o` compiled with clang, the program will use that automatically. Place it in the project root or specify it with `--ebpf`:

```bash
sudo ./target/release/kfree_skb --verbose --ebpf ./kfree_skb.bpf.o
```

## Next Steps

Once the eBPF program is built, you can run the tracer:

```bash
sudo ./target/release/kfree_skb --verbose
```

See [QUICKSTART.md](QUICKSTART.md) for more usage examples.
