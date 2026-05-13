#![no_std]
#![no_main]

use aya_bpf::{
    bindings::*,
    helpers::*,
    maps::Array,
    programs::TracePointContext,
    BpfContext,
};
use aya_log_ebpf::info;

// BPF map for counting socket drops
#[map]
static SOCKET_DROP_COUNTS: Array<u64> = Array::with_max_entries(6, 0);

// Socket drop reason indices
const REASON_NO_SOCKET: u32 = 0;
const REASON_SOCKET_CLOSE: u32 = 1;
const REASON_SOCKET_FILTER: u32 = 2;
const REASON_SOCKET_RCVBUFF: u32 = 3;
const REASON_SOCKET_BACKLOG: u32 = 4;
const REASON_PACKET_SOCK_ERROR: u32 = 5;

// SKB drop reasons from kernel
const SKB_DROP_REASON_NO_SOCKET: u32 = 2;
const SKB_DROP_REASON_SOCKET_CLOSE: u32 = 3;
const SKB_DROP_REASON_SOCKET_FILTER: u32 = 4;
const SKB_DROP_REASON_SOCKET_RCVBUFF: u32 = 5;
const SKB_DROP_REASON_SOCKET_BACKLOG: u32 = 6;
const SKB_DROP_REASON_PACKET_SOCK_ERROR: u32 = 11;

#[repr(C)]
struct TraceEventRawKfreeSKB {
    common_type: u16,
    common_flags: u8,
    common_preempt_count: u8,
    common_pid: u32,
    skbaddr: u64,
    protocol: u16,
    reason: u32,
}

fn count_socket_drop(idx: u32) {
    if let Some(count) = SOCKET_DROP_COUNTS.get_ptr_mut(idx) {
        unsafe {
            *count = count.wrapping_add(1);
        }
    }
}

#[tracepoint]
pub fn trace_kfree_skb(ctx: TracePointContext) -> u32 {
    match unsafe { try_trace_kfree_skb(ctx) } {
        Ok(ret) => ret,
        Err(_e) => 1,
    }
}

unsafe fn try_trace_kfree_skb(ctx: TracePointContext) -> Result<u32, i64> {
    let event = ctx.read_kernel_struct::<TraceEventRawKfreeSKB>(ctx.as_ptr())?;
    let reason = event.reason;
    let reason_name: &[u8] = b"OTHER";
    let mut idx: u32 = u32::MAX;

    let (idx_val, reason_str) = match reason {
        SKB_DROP_REASON_NO_SOCKET => {
            idx = REASON_NO_SOCKET;
            (REASON_NO_SOCKET, "NO_SOCKET")
        }
        SKB_DROP_REASON_SOCKET_CLOSE => {
            idx = REASON_SOCKET_CLOSE;
            (REASON_SOCKET_CLOSE, "SOCKET_CLOSE")
        }
        SKB_DROP_REASON_SOCKET_FILTER => {
            idx = REASON_SOCKET_FILTER;
            (REASON_SOCKET_FILTER, "SOCKET_FILTER")
        }
        SKB_DROP_REASON_SOCKET_RCVBUFF => {
            idx = REASON_SOCKET_RCVBUFF;
            (REASON_SOCKET_RCVBUFF, "SOCKET_RCVBUFF")
        }
        SKB_DROP_REASON_SOCKET_BACKLOG => {
            idx = REASON_SOCKET_BACKLOG;
            (REASON_SOCKET_BACKLOG, "SOCKET_BACKLOG")
        }
        SKB_DROP_REASON_PACKET_SOCK_ERROR => {
            idx = REASON_PACKET_SOCK_ERROR;
            (REASON_PACKET_SOCK_ERROR, "PACKET_SOCK_ERROR")
        }
        _ => (u32::MAX, "OTHER"),
    };

    if idx != u32::MAX {
        count_socket_drop(idx);
    }

    info!(&ctx, "kfree_skb: reason={} ({})", reason, reason_str);

    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
