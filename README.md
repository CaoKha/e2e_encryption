# E2E Encryption SDK and CLI

This project is an End-to-End Encryption (E2EE) Software Development Kit (SDK) and
Command Line Interface (CLI) implemented in Rust. It provides tools and libraries
for secure communication. Currently, it supports only Linux desktop.

## Prerequisites

Before you begin, ensure you have the following installed on your system:

1. **Rust**: This project is built with Rust. If you don't have Rust installed, you can install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2. **Cargo**: Cargo is Rust's package manager and build tool. It comes pre-installed with Rust.

3. **Make**: This project uses a Makefile to simplify the build process. Make sure you have Make installed on your system.

## Building the Project

This project uses a Makefile to manage various build targets. Here are the main commands you can use:

### Desktop Build

To build for Linux desktop X86_64 architecture:

```bash
make build-desktop-x86_64-unknown-linux-gnu
```

To build for Linux desktop with FFI support:

```bash
make build-desktop-ffi-x86_64-unknown-linux-gnu
```

### Running Tests

To run all tests:

```bash
make test
```

To run specific test suites:

```bash
make test-e2ee-lib
make test-e2ee-doc
```

### Running Examples

The project includes several examples demonstrating its functionality:

```bash
make example-e2ee-simple
make example-e2ee-key-generation
make example-e2ee-server-encrypt MESSAGE="Your message" SIZE=bit1024
make example-e2ee-client-encrypt MESSAGE="Your message"
make example-e2ee-server-decrypt
```

### Cleaning Build Artifacts

To clean all build artifacts:

```bash
make clean
```

## Getting Help

For a full list of available make targets and their descriptions:

```bash
make help
```

## Project Structure

```
.
├── Cargo.lock
├── Cargo.toml
├── crates
│   ├── cli
│   │   └── e2ee
│   └── lib
│       └── e2ee
├── Justfile
├── Makefile
├── README.md
├── rustfmt.toml
└── rust-toolchain.toml
```

The main SDK code is located in the `crates/lib/e2ee` directory. The `crates/cli/e2ee` directory contains command-line interface tools for the SDK.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

[Insert your chosen license here]
