#include "vmlinux.h"
#include <bpf/bpf_helpers.h>

// Define the event structure to pass data to userspace
struct {
  __uint(type, BPF_MAP_TYPE_RINGBUF);
  __uint(max_entries, 256 * 1024);
} rb SEC(".maps");

SEC("tracepoint/skb/kfree_skb")
int trace_kfree_skb(struct trace_event_raw_kfree_skb *ctx) {
  int reason = ctx->reason;
  const char *reason_name = "OTHER";

  switch (reason) {
    case SKB_DROP_REASON_NO_SOCKET:
      reason_name = "NO_SOCKET";
      break;
    case SKB_DROP_REASON_SOCKET_CLOSE:
      reason_name = "SOCKET_CLOSE";
      break;
    case SKB_DROP_REASON_SOCKET_FILTER:
      reason_name = "SOCKET_FILTER";
      break;
    case SKB_DROP_REASON_SOCKET_RCVBUFF:
      reason_name = "SOCKET_RCVBUFF";
      break;
    case SKB_DROP_REASON_SOCKET_BACKLOG:
      reason_name = "SOCKET_BACKLOG";
      break;
    case SKB_DROP_REASON_PACKET_SOCK_ERROR:
      reason_name = "PACKET_SOCK_ERROR";
      break;
  }

  bpf_printk("kfree_skb: reason=%d (%s)\n", reason, reason_name);
  return 0;
}

char LICENSE[] SEC("license") = "GPL";
