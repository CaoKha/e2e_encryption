//! # E2EE Library
//!
//! This library provides an end-to-end encryption (E2EE) system using RSA encryption, designed for both client and server usage.
//! It includes encryption and decryption functionalities, making it ideal for secure communication between clients and servers.
//!
//! ## Modules
//!
//! - `client`: Contains the client-side encryption logic that uses only the public key for encryption.
//! - `server`: Contains the server-side encryption and decryption logic that requires both private and public keys.
//! - `ffi` (optional): Provides a foreign function interface (FFI) for integrating the encryption system with other platforms.
//!
//! ## Usage Examples
//!
//! ### Initializing the Server-Side E2EE
//!
//! To create an `E2ee` instance on the server side, both the private and public keys are required.
//!
//! ```rust
//! use e2ee::server::E2ee;
//!
//! const PRIVATE_KEY_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/files/private.pem");
//! const PUBLIC_KEY_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/files/public.pem");
//!
//! // Load the private and public keys from PEM files.
//! let private_key_pem = std::fs::read_to_string(PRIVATE_KEY_PATH).expect("Failed to read private key file");
//! let public_key_pem = std::fs::read_to_string(PUBLIC_KEY_PATH).expect("Failed to read public key file");
//!
//! // Initialize the `E2ee` instance with both private and public keys.
//! let e2ee_server = E2ee::new_from_pem(private_key_pem, public_key_pem).expect("Failed to create E2ee server instance");
//! ```
//!
//! ## Features
//!
//! - **`ffi`**: Enable the `ffi` feature to include the foreign function interface for cross-platform support.
pub mod client;
#[cfg(feature = "ffi")]
pub mod ffi;
pub mod server;
