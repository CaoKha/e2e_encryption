# Makefile for E2EE SDK

# Phony targets
.PHONY: all clean \
				build-desktop-x86_64-unknown-linux-gnu build-desktop-ffi-x86_64-unknown-linux-gnu \
				build-desktop-x86_64-pc-windows-gnu \
        example-e2ee-simple example-e2ee-key-generation example-e2ee-server-encrypt \
				example-e2ee-client-encrypt example-e2ee-server-decrypt \
				test test-e2ee-lib test-e2ee-doc \
				test-cross-x86_64-unknown-linux-gnu \
				test-cross-x86_64-pc-windows-gnu \

# Default target
all: build-desktop-x86_64-unknown-linux-gnu

# Check if cross is installed, if not, install it
check-cross:
	@if ! command -v cross > /dev/null; then \
		echo "cross is not installed. Installing now..."; \
		cargo install cross; \
	fi

# Clean build artifacts
clean:
	cargo clean

# Build targets (CLI and E2EE Lib)
build-desktop-x86_64-unknown-linux-gnu:
	cross build --release --target x86_64-unknown-linux-gnu

build-desktop-ffi-x86_64-unknown-linux-gnu:
	cross build --release --features ffi --target x86_64-unknown-linux-gnu

build-desktop-x86_64-pc-windows-gnu:
	cross build --release --target x86_64-pc-windows-gnu

# Test targets
test-e2ee-lib:
	cargo test -p e2ee --tests

test-e2ee-doc:
	cargo test -p e2ee --doc

test: test-e2ee-lib test-e2ee-doc

test-cross-x86_64-unknown-linux-gnu:
	cross test -p e2ee --tests --target x86_64-unknown-linux-gnu

test-cross-x86_64-pc-windows-gnu:
	cross test -p e2ee --tests --target x86_64-pc-windows-gnu # bug: bcryptprimitives.dll (needed for encryption) not found (wine doesn't include it. But real Windows system does)

# Example targets (unchanged)
example-e2ee-simple:
	cargo run -p e2ee --example e2ee_simple

example-e2ee-key-generation:
	cargo run -p e2ee --example e2ee_key_generation

example-e2ee-server-encrypt:
	@if [ -z "$(MESSAGE)" ] || [ -z "$(SIZE)" ]; then \
		echo "Usage: make example-e2ee-server-encrypt MESSAGE=\"your message\" SIZE=<size>"; \
		exit 1; \
	fi
	cargo run -p e2ee --example e2ee_server_encrypt -- -m "$(MESSAGE)" -s $(SIZE)

example-e2ee-client-encrypt:
	@if [ -z "$(MESSAGE)" ]; then \
		echo "Usage: make example-e2ee-client-encrypt MESSAGE=\"your message\""; \
		exit 1; \
	fi
	cargo run -p e2ee --example e2ee_client_encrypt -- -m "$(MESSAGE)"

example-e2ee-server-decrypt:
	cargo run -p e2ee --example e2ee_server_decrypt

# Help target
help:
	@echo "Available targets:"
	@echo "  all                                    		- Default target, builds for desktop"
	@echo "  clean                                  		- Clean build artifacts"
	@echo "  test                                   		- Run all tests"
	@echo "  test-e2ee-lib                          		- Run e2ee library tests"
	@echo "  test-e2ee-doc                          		- Run e2ee documentation tests"
	@echo "  test-cross-x86_64-unknown-linux-gnu    		- Run e2ee library tests against x86_64-unknown-linux-gnu architecture"
	@echo "  test-cross-x86_64-pc-windows-gnu 			- Run e2ee library tests against x86_64-pc-windows-gnu architecture"
	@echo "  build-desktop-x86_64-unknown-linux-gnu 		- Build for desktop"
	@echo "  build-desktop-ffi-x86_64-unknown-linux-gnu 		- Build for desktop with FFI feature"
	@echo "  build-desktop-x86_64-pc-windows-gnu  			- Build for desktop on x86_64 Windows"
	@echo "  example-e2ee-simple                    		- Run simple e2ee example"
	@echo "  example-e2ee-key-generation            		- Run e2ee key generation example"
	@echo "  example-e2ee-server-encrypt            		- Run e2ee server encrypt example"
	@echo "  example-e2ee-client-encrypt            		- Run e2ee client encrypt example"
	@echo "  example-e2ee-server-decrypt            		- Run e2ee server decrypt example"
	@echo ""
	@echo "Usage examples:"
	@echo "  make example-e2ee-server-encrypt MESSAGE=\"Hello, World!\" SIZE=bit2048"
	@echo "  make example-e2ee-client-encrypt MESSAGE=\"Secret message\""
