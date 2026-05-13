.PHONY: build release test clean fmt check doc help

# Default target
help:
	@echo "kfree_skb Tracer - Rust Edition"
	@echo ""
	@echo "Available targets:"
	@echo "  build       - Build debug version"
	@echo "  release     - Build optimized release version"
	@echo "  ebpf        - Build only eBPF kernel program"
	@echo "  test        - Run all tests"
	@echo "  test-run    - Run the tracer with test traffic (requires sudo)"
	@echo "  run         - Run the tracer continuously (requires sudo)"
	@echo "  stats       - Show statistics only (requires sudo)"
	@echo "  fmt         - Format code"
	@echo "  check       - Run clippy linter"
	@echo "  doc         - Build and open documentation"
	@echo "  clean       - Clean build artifacts"

# Build targets
build:
	@echo "Building debug version..."
	cargo build

release:
	@echo "Building release version..."
	cargo build --release

ebpf:
	@echo "Building eBPF kernel program..."
	cd ebpf && cargo build --target bpf --release

# Testing
test:
	@echo "Running tests..."
	cargo test

test-run: release
	@echo "Running tracer with test traffic..."
	sudo ./target/release/kfree_skb --verbose &
	sleep 1
	ping -c 10 127.0.0.1 >/dev/null 2>&1
	sleep 1
	pkill -f "kfree_skb --verbose" || true
	sudo ./target/release/kfree_skb --stats

# Runtime targets
run: release
	@echo "Starting kfree_skb tracer (press Ctrl+C to stop)..."
	sudo ./target/release/kfree_skb --verbose

stats: release
	@echo "Collecting statistics..."
	sudo ./target/release/kfree_skb --stats

# Code quality
fmt:
	@echo "Formatting code..."
	cargo fmt

check:
	@echo "Running clippy..."
	cargo clippy -- -D warnings

doc:
	@echo "Building documentation..."
	cargo doc --no-deps --open

# Cleanup
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

.PHONY: build release ebpf test test-run run stats fmt check doc clean help
