use base64::{engine::general_purpose, Engine};
use error::PublicE2eeResult;
use rsa::{
    pkcs8::DecodePublicKey, rand_core::OsRng, sha2::Sha256, Oaep, RsaPublicKey,
};

mod error;

/// A struct representing the End-to-End Encryption (E2EE) system on the client side.
///
/// This struct is used for encryption operations on the client side. It includes:
///
/// - **Encryption**: Encrypts messages using the public key provided by the server.
///
/// The client-side E2EE system requires only the public key to function correctly. The public key is used
/// for encrypting messages before they are sent to the server.
///
/// The `PublicE2ee` struct includes the following fields:
///
/// - `public_key`: The RSA public key used for encrypting messages.
/// - `public_key_pem`: The PEM-encoded public key as a string.
///
/// # Examples
///
/// ```
/// use e2ee::client::PublicE2ee;
///
/// const PUBLIC_KEY_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/files/public.pem");
/// // Create a new PublicE2ee instance from a PEM-encoded public key.
/// let public_key_pem = std::fs::read_to_string(PUBLIC_KEY_PATH).expect("Failed to read public key file");
/// let e2ee_client = PublicE2ee::new(public_key_pem.to_string()).expect("Failed to create PublicE2ee instance");
///
/// // Encrypt a message.
/// let encrypted_message = e2ee_client.encrypt("Secret message").expect("Failed to encrypt message");
/// println!("Encrypted Message: {}", encrypted_message);
/// ```
///
/// # Errors
///
/// The struct's methods may return errors if encryption operations fail.
#[derive(Debug)]
pub struct PublicE2ee {
    public_key: RsaPublicKey,
    public_key_pem: String,
}

impl PublicE2ee {
    /// Creates a new `PublicE2ee` instance from a PEM-encoded public key.
    ///
    /// This method takes a PEM-encoded RSA public key as input and constructs a `PublicE2ee` instance
    /// that can be used to perform encryption operations. The PEM string is parsed into an `RsaPublicKey`
    /// object, which is then used for encryption.
    ///
    /// # Arguments
    ///
    /// * `public_key_pem` - A `String` containing the PEM-encoded RSA public key. The public key must be
    ///   a valid PEM-encoded RSA key, as this method will attempt to decode it using the RSA PKCS#8 standard.
    ///
    /// # Returns
    ///
    /// This method returns a `PublicE2eeResult<Self>`, which contains:
    /// - An `Ok(PublicE2ee)` variant if the `public_key_pem` is successfully parsed and the `PublicE2ee`
    ///   instance is created.
    /// - An `Err` variant if an error occurs during the parsing of the PEM string or if the public key is invalid.
    ///
    /// # Errors
    ///
    /// The function may return an error in the following cases:
    /// - If the `public_key_pem` is not a valid PEM-encoded RSA public key.
    /// - If the `public_key_pem` string cannot be parsed or decoded correctly. This includes cases where the
    ///   provided key is not in the expected format (e.g., it is malformed or encrypted).
    ///
    /// # Examples
    ///
    /// ```
    /// use e2ee::client::PublicE2ee;
    ///
    /// const PUBLIC_KEY_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/files/public.pem");
    /// // Load the public key from a PEM file.
    /// let public_key_pem = std::fs::read_to_string(PUBLIC_KEY_PATH).expect("Failed to read public key file");
    ///
    /// // Create a new `PublicE2ee` instance with the PEM-encoded public key.
    /// let e2ee_client = PublicE2ee::new(public_key_pem.to_string()).expect("Failed to create PublicE2ee instance");
    /// ```
    ///
    /// # Safety
    ///
    /// This method is safe to use as long as the provided `public_key_pem` is a valid PEM-encoded RSA public key.
    /// Ensure that the public key string is properly formatted and originates from a trusted source. Passing an
    /// invalid or corrupted PEM string will result in an error.
    pub fn new(public_key_pem: String) -> PublicE2eeResult<Self> {
        let public_key = RsaPublicKey::from_public_key_pem(&public_key_pem)?;
        Ok(Self {
            public_key,
            public_key_pem,
        })
    }

    /// Encrypts a message using the public key.
    ///
    /// This function takes a plaintext message and encrypts it using the RSA public key
    /// stored in the `PublicE2ee` struct. The encryption is performed with Optimal Asymmetric
    /// Encryption Padding (OAEP) and a random number generator.
    ///
    /// # Arguments
    ///
    /// * `message` - The plaintext message to encrypt. This should be a string slice (`&str`).
    ///
    /// # Returns
    ///
    /// This function returns the encrypted message as a base64-encoded string. If encryption fails,
    /// it returns an error.
    ///
    /// # Errors
    ///
    /// The function may return an error if the encryption process fails. Possible reasons include:
    /// - Issues with the public key or padding scheme.
    /// - Problems with the random number generator.
    ///
    /// # Examples
    ///
    /// ```
    /// use e2ee::client::PublicE2ee;
    ///
    /// // Example public key PEM (replace with a valid key).
    /// const PUBLIC_KEY_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/files/public.pem");
    /// let public_key_pem = std::fs::read_to_string(PUBLIC_KEY_PATH).expect("Failed to read public key file");
    /// let e2ee_client = PublicE2ee::new(public_key_pem.to_string()).expect("Failed to create PublicE2ee instance");
    ///
    /// // Encrypt a message.
    /// let encrypted_message = e2ee_client.encrypt("Secret message").expect("Failed to encrypt message");
    /// println!("Encrypted Message: {}", encrypted_message);
    /// ```
    ///
    /// # Safety
    ///
    /// Ensure that the `PublicE2ee` instance is correctly initialized with a valid public key before
    /// calling this method. Passing an invalid or improperly initialized instance may lead to errors.
    pub fn encrypt(&self, message: &str) -> PublicE2eeResult<String> {
        let mut rng = OsRng;
        let padding = Oaep::new::<Sha256>();
        let encrypted_data =
            self.public_key
                .encrypt(&mut rng, padding, message.as_bytes())?;
        Ok(general_purpose::STANDARD_NO_PAD.encode(encrypted_data))
    }

    /// Retrieves the PEM-encoded public key.
    pub fn get_public_key_pem(&self) -> &str {
        &self.public_key_pem
    }
}

#[cfg(test)]
mod tests {
    use super::PublicE2ee;
    use std::fs;

    const PUBLIC_KEY_PATH: &str =
        concat!(env!("CARGO_MANIFEST_DIR"), "/files/public.pem");

    #[test]
    fn test_public_e2ee_new() {
        // Read the public key from a file.
        let public_key_pem = fs::read_to_string(PUBLIC_KEY_PATH)
            .expect("Failed to read public key file");

        // Attempt to create a new `PublicE2ee` instance.
        let e2ee_client = PublicE2ee::new(public_key_pem.to_string());

        // Assert that the instance creation is successful.
        assert!(e2ee_client.is_ok(), "Failed to create PublicE2ee instance");
    }

    #[test]
    fn test_public_e2ee_encrypt() {
        // Read the public key from a file.
        let public_key_pem = fs::read_to_string(PUBLIC_KEY_PATH)
            .expect("Failed to read public key file");

        // Create a new `PublicE2ee` instance.
        let e2ee_client = PublicE2ee::new(public_key_pem.to_string())
            .expect("Failed to create PublicE2ee instance");

        // Message to encrypt.
        let message = "Secret message";

        // Attempt to encrypt the message.
        let encrypted_message = e2ee_client.encrypt(message);

        // Assert that encryption was successful.
        assert!(encrypted_message.is_ok(), "Failed to encrypt message");

        // Additional check: Ensure the encrypted message is not the same as the input message.
        let encrypted_message_str = encrypted_message.unwrap();
        assert_ne!(
            message, encrypted_message_str,
            "Encrypted message should differ from the original message"
        );
    }

    #[test]
    fn test_public_e2ee_get_public_key_pem() {
        // Read the public key from a file.
        let public_key_pem = fs::read_to_string(PUBLIC_KEY_PATH)
            .expect("Failed to read public key file");

        // Create a new `PublicE2ee` instance.
        let e2ee_client = PublicE2ee::new(public_key_pem.clone())
            .expect("Failed to create PublicE2ee instance");

        // Retrieve the public key PEM and ensure it matches the original.
        assert_eq!(
            e2ee_client.get_public_key_pem(),
            public_key_pem,
            "Retrieved public key PEM does not match the original"
        );
    }
}
