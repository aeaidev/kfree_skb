#!/bin/sh

sudo -n sh -c 'rm -rf /sys/fs/bpf/kfree_skb_test /sys/fs/bpf/kfree_skb_maps 2>/dev/null; bpftool prog loadall /home/igor/projects/warp_kfree_skb/kfree_skb.bpf.o /sys/fs/bpf/kfree_skb_test pinmaps /sys/fs/bpf/kfree_skb_maps autoattach; ping -c 10 127.0.0.1 >/dev/null 2>&1; python3 /home/igor/projects/warp_kfree_skb/print_socket_drop_stats.py; rm -rf /sys/fs/bpf/kfree_skb_test /sys/fs/bpf/kfree_skb_maps 2>/dev/null'
