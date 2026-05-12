#!/usr/bin/env python3
import json
import subprocess
import sys

MAP_PATH = "/sys/fs/bpf/kfree_skb_maps/socket_drop_counts"
REASONS = [
    "NO_SOCKET",
    "SOCKET_CLOSE",
    "SOCKET_FILTER",
    "SOCKET_RCVBUFF",
    "SOCKET_BACKLOG",
    "PACKET_SOCK_ERROR",
]


def lookup_count(idx: int) -> int:
    key_bytes = [(idx >> (8 * i)) & 0xFF for i in range(4)]
    cmd = [
        "bpftool",
        "-j",
        "map",
        "lookup",
        "pinned",
        MAP_PATH,
        "key",
        "hex",
        *(f"{b:02x}" for b in key_bytes),
    ]
    res = subprocess.run(cmd, capture_output=True, text=True)
    if res.returncode != 0:
        return 0
    try:
        data = json.loads(res.stdout)
    except json.JSONDecodeError:
        return 0
    value = data.get("value", [])
    if isinstance(value, int):
        return value
    if isinstance(value, list):
        count = 0
        for i, byte in enumerate(value[:8]):
            byte_val = int(byte, 0) if isinstance(byte, str) else int(byte)
            count |= (byte_val & 0xFF) << (8 * i)
        return count
    return 0


def main() -> int:
    print("Socket drop statistics:")
    total = 0
    for idx, name in enumerate(REASONS):
        count = lookup_count(idx)
        total += count
        print(f"- {name}: {count}")
    print(f"- TOTAL: {total}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
