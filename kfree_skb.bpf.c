#include "vmlinux.h"
#include <bpf/bpf_helpers.h>

struct {
  __uint(type, BPF_MAP_TYPE_ARRAY);
  __uint(max_entries, 6);
  __type(key, __u32);
  __type(value, __u64);
} socket_drop_counts SEC(".maps");

enum socket_reason_idx {
  REASON_NO_SOCKET = 0,
  REASON_SOCKET_CLOSE = 1,
  REASON_SOCKET_FILTER = 2,
  REASON_SOCKET_RCVBUFF = 3,
  REASON_SOCKET_BACKLOG = 4,
  REASON_PACKET_SOCK_ERROR = 5,
};

static __always_inline void count_socket_drop(__u32 idx) {
  __u64 *count = bpf_map_lookup_elem(&socket_drop_counts, &idx);
  if (count) {
    __sync_fetch_and_add(count, 1);
  }
}

SEC("tracepoint/skb/kfree_skb")
int trace_kfree_skb(struct trace_event_raw_kfree_skb *ctx) {
  int reason = ctx->reason;
  __u32 idx = (__u32)-1;
  const char *reason_name = "OTHER";

  switch (reason) {
    case SKB_DROP_REASON_NO_SOCKET:
      idx = REASON_NO_SOCKET;
      reason_name = "NO_SOCKET";
      break;
    case SKB_DROP_REASON_SOCKET_CLOSE:
      idx = REASON_SOCKET_CLOSE;
      reason_name = "SOCKET_CLOSE";
      break;
    case SKB_DROP_REASON_SOCKET_FILTER:
      idx = REASON_SOCKET_FILTER;
      reason_name = "SOCKET_FILTER";
      break;
    case SKB_DROP_REASON_SOCKET_RCVBUFF:
      idx = REASON_SOCKET_RCVBUFF;
      reason_name = "SOCKET_RCVBUFF";
      break;
    case SKB_DROP_REASON_SOCKET_BACKLOG:
      idx = REASON_SOCKET_BACKLOG;
      reason_name = "SOCKET_BACKLOG";
      break;
    case SKB_DROP_REASON_PACKET_SOCK_ERROR:
      idx = REASON_PACKET_SOCK_ERROR;
      reason_name = "PACKET_SOCK_ERROR";
      break;
  }

  if (idx != (__u32)-1) {
    count_socket_drop(idx);
  }

  bpf_printk("kfree_skb: reason=%d (%s)\n", reason, reason_name);
  return 0;
}

char LICENSE[] SEC("license") = "GPL";
