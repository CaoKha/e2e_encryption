# E2E Encryption SDK and CLI

[![Rust](https://github.com/CaoKha/e2e_encryption/actions/workflows/rust.yml/badge.svg)](https://github.com/CaoKha/e2e_encryption/actions/workflows/rust.yml)
[![Release](https://github.com/CaoKha/e2e_encryption/actions/workflows/release.yml/badge.svg)](https://github.com/CaoKha/e2e_encryption/actions/workflows/release.yml)
[![Docs](https://github.com/CaoKha/e2e_encryption/actions/workflows/docs.yml/badge.svg)](https://github.com/CaoKha/e2e_encryption/actions/workflows/docs.yml)

This project is my attempt to build an End-to-End Encryption (E2EE)
Software Development Kit (SDK) and Command Line Interface (CLI)
implemented in Rust. It uses [RSA](https://docs.rs/rsa/latest/rsa/) libraries to
perform encryption and decryption. Currently, it supports only Linux, Windows
and macOS desktop.

## Prerequisites

Before you begin, ensure you have the following installed on your system:

1. **Rust**: This project is built with Rust. If you don't have Rust installed,
   you can install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2. **Cargo**: Cargo is Rust's package manager and build tool.
   It comes pre-installed with Rust.

3. **Make**: This project uses a Makefile to simplify many run commands.
   Make sure you have Make installed on your system.

## Building from source

### Desktop Build

To build for Linux desktop X86_64 architecture:

```bash
make build-desktop-x86_64-unknown-linux-gnu
```

To build for Linux desktop with FFI support:

```bash
make build-desktop-ffi-x86_64-unknown-linux-gnu
```

The make command uses cargo under the hood in order to build the project.
The executable and libraries are found in the `target` folder.
You now can just run the executable from there and enjoy the CLI!
For the SDK libraries, I have two options: rather I publish the library to crates.io (which I do not)
or you can just add the library to your **Cargo.toml** file like this:

```toml
# a commit with a particular tag
e2ee = { git = "https://github.com/CaoKha/e2e_encryption.git", tag = "0.1.2" }
```

To see the full list of available make targets and their descriptions:

```bash
make help
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

To run tests via cross-rs
(Cross compilation and do the tests in separate container using Docker or Podman.
Checkout their [github](https://github.com/cross-rs/cross) for more details):

- Linux desktop X86_64 architecture:

```bash
make test-cross-x86_64-unknown-linux-gnu
```

- Windows X86_64 architecture:
  (Bug: **bcryptprimitives.dll** (needed for encryption) not found.
  Wine doesn't include it)

```bash
make test-cross-x86_64-pc-windows-gnu
```

### Running SDK Examples

The project includes several examples demonstrating its SDK functionality:

```bash
make example-e2ee-simple
make example-e2ee-key-generation
make example-e2ee-server-encrypt MESSAGE="Your message" SIZE=bit1024
make example-e2ee-client-encrypt MESSAGE="Your message"
make example-e2ee-server-decrypt
```

### Cleaning Build Artifacts

To clean all build artifacts (This just deletes the `target` folder):

```bash
make clean
```

## Cross-compiling SDK to iOS and Android

If you need to cross-compile the Rust library to iOS or Android ,
[Dinghy](https://github.com/sonos/dinghy) is a great tool. (Since I am working
on a Linux machine and I do not have the required phone simulator SDKs,
the setup is a pain in the butt).

Dinghy simplifies cross-compilation by providing
a set of tools and configurations to build Rust projects for mobile platforms.
For more information on how to use dinghy, visit the [dinghy GitHub repository](https://github.com/sonos/dinghy).

## Downloading Precompiled CLI Binaries

To use the precompiled binaries for the CLI, you can download the appropriate version
for your operating system from the [releases](https://github.com/CaoKha/e2e_encryption/releases)
page. The available binaries are:

```text
  Windows: .msi installer
  Linux: .tar.xz archive
  MacOS: .tar.xz archive
```

There are other options too, go to the [releases](https://github.com/CaoKha/e2e_encryption/releases)
page for more details.

### Installation Instructions

- **Windows**: Download the .msi file and run the installer to set up the CLI on
  your system.

- **Linux** or **MacOS**: Download the .tar.xz file, extract it,
  and place the binary in a directory
  included in your PATH:

```bash
# MacOS
tar -xvf path/to/e2ee-cli-aarch64-apple-darwin.tar.xz
sudo mv e2ee-cli-aarch64-apple-darwin/e2ee-cli /usr/local/bin/
```

```bash
# Linux
tar -xvf path/to/e2ee-cli-x86_64-unknown-linux-gnu.tar.xz
sudo mv e2ee-cli-x86_64-unknown-linux-gnu/e2ee-cli /usr/local/bin/
```

Or just run it directly from the extracted folder (recommended for testing purpose):

```bash
# MacOS
tar -xvf path/to/e2ee-cli-aarch64-apple-darwin.tar.xz
cd e2ee-cli-aarch64-apple-darwin
./e2ee-cli -h
```

```bash
# Linux
tar -xvf path/to/e2ee-cli-x86_64-unknown-linux-gnu.tar.xz
cd e2ee-cli-x86_64-unknown-linux-gnu
./e2ee-cli -h
```

## CLI Usage

```bash
❯ e2ee-cli -h
CLI tool to encrypt and decrypt messages using RSA encryption

Usage: e2ee-cli <COMMAND>

Commands:
  generate-keys  Generate a new pair of RSA keys and save them to files
  encrypt        Encrypt a message using a public RSA key
  decrypt        Decrypt a ciphertext using a private RSA key
  help           Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version
```

## Project Structure

```text
├── Cargo.lock
├── Cargo.toml
├── crates
│   ├── cli
│   │   └── e2ee
│   │       ├── Cargo.toml
│   │       └── src
│   │           └── main.rs
│   └── lib
│       └── e2ee
│           ├── Cargo.toml
│           ├── examples
│           │   ├── e2ee_client_encrypt.rs
│           │   ├── e2ee_key_generation.rs
│           │   ├── e2ee_server_decrypt.rs
│           │   ├── e2ee_server_encrypt.rs
│           │   └── e2ee_simple.rs
│           ├── files
│           │   ├── private.pem
│           │   └── public.pem
│           └── src
│               ├── client
│               │   └── error.rs
│               ├── client.rs
│               ├── ffi.rs
│               ├── lib.rs
│               ├── server
│               │   └── error.rs
│               └── server.rs
├── deny.toml
├── Justfile
├── LICENSE-MIT
├── Makefile
├── README.md
├── rustfmt.toml
└── rust-toolchain.toml
```

The main SDK code is located in the `crates/lib/e2ee` directory.
The `crates/cli/e2ee` directory contains command-line interface tools for the SDK.
Document is also available at [https://caokha.github.io/e2e_encryption/](https://caokha.github.io/e2e_encryption/).
You can also find it in the about section of the project.

## License

[MIT](./LICENSE-MIT)
