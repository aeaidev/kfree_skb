# kfree_skb BPF Tracepoint Example
This project contains a minimal eBPF tracepoint program that hooks `tracepoint/skb/kfree_skb` and prints skb drop reasons via `bpf_printk`.

## Files
- `kfree_skb.bpf.c` — eBPF program source
- `kfree_skb.bpf.o` — compiled eBPF object
- `vmlinux.h` — BTF-generated kernel type definitions used by the program

## Prerequisites
- `clang` with BPF target support
- `bpftool`
- Kernel with BPF support
- Root privileges (or equivalent capabilities) for load/attach and trace reads
## Generate `vmlinux.h`
Run this once (or after kernel upgrades) to generate BTF-based kernel type definitions:
```bash path=null start=null
bpftool btf dump file /sys/kernel/btf/vmlinux format c > vmlinux.h
```

## Build
```bash path=null start=null
clang -g -O2 -target bpf -c kfree_skb.bpf.c -o kfree_skb.bpf.o
```

## Run (load + auto-attach)
```bash path=null start=null
sudo bpftool prog loadall kfree_skb.bpf.o /sys/fs/bpf/kfree_skb_test autoattach
```

## Observe output
In one terminal:
```bash path=null start=null
sudo cat /sys/kernel/tracing/trace_pipe
```

In another terminal, generate traffic:
```bash path=null start=null
ping -c 10 127.0.0.1
```

You should see lines like:
```text path=null start=null
bpf_trace_printk: kfree_skb: reason=3 (NO_SOCKET)
bpf_trace_printk: kfree_skb: reason=4 (SOCKET_CLOSE)
```

## Cleanup
```bash path=null start=null
sudo rm -rf /sys/fs/bpf/kfree_skb_test
```

## Notes
- The program currently decodes socket-related drop reasons by name and prints other reasons as `OTHER` with their numeric ID.
- Extend the switch in `kfree_skb.bpf.c` if you want full enum-name decoding for all `skb_drop_reason` values.
