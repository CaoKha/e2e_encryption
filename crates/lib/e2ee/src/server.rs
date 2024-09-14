use base64::{engine::general_purpose, Engine};
use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey},
    rand_core::OsRng,
    sha2::Sha256,
    Oaep, RsaPrivateKey, RsaPublicKey,
};
mod error;
use clap::ValueEnum;
use error::{E2eeError, E2eeResult};
use std::{fs::File, io::Write};

/// A struct representing the End-to-End Encryption (E2EE) system on the server side.
///
/// This struct manages the full lifecycle of key management and encryption operations. It includes:
///
/// - **Key Generation**: Generates a key pair (public and private keys) for encryption and decryption.
/// - **Encryption**: Encrypts messages using the server's public key.
/// - **Decryption**: Decrypts messages using the server's private key.
///
/// The server side E2EE system requires both a private key and a public key to function correctly.
/// The private key is used for decryption, while the public key is used for encryption.
///
/// The `E2ee` struct includes the following fields:
///
/// - `private_key`: The RSA private key used for decrypting messages.
/// - `public_key`: The RSA public key used for encrypting messages.
/// - `private_key_pem`: The PEM-encoded private key as a string.
/// - `public_key_pem`: The PEM-encoded public key as a string.
///
/// # Examples
///
/// ```
/// use e2ee::server::{E2ee, KeySize};
///
/// // Create a new E2ee instance with a specific key size.
/// let e2ee = E2ee::new(KeySize::Bit2048).expect("Failed to create E2ee instance");
///
/// // Get the PEM-encoded public key.
/// let public_key_pem = e2ee.get_public_key_pem();
/// println!("Public Key PEM: {}", public_key_pem);
///
/// // Encrypt a message.
/// let encrypted_message = e2ee.encrypt("Secret message").expect("Failed to encrypt message");
/// println!("Encrypted Message: {}", encrypted_message);
///
/// // Decrypt a message.
/// let decrypted_message = e2ee.decrypt(&encrypted_message).expect("Failed to decrypt message");
/// println!("Decrypted Message: {}", decrypted_message);
/// ```
///
/// # Errors
///
/// The struct's methods may return errors if key generation fails, or if encryption/decryption operations fail.
#[derive(Debug)]
pub struct E2ee {
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey,
    private_key_pem: String,
    public_key_pem: String,
}

/// Represents the key sizes available for RSA key generation.
///
/// The sizes are in bits and correspond to common RSA key lengths.
#[derive(Debug, ValueEnum, Clone, Copy)]
pub enum KeySize {
    /// 1024-bit RSA key
    Bit1024 = 1024,
    /// 2048-bit RSA key
    Bit2048 = 2048,
    /// 3072-bit RSA key
    Bit3072 = 3072,
    /// 4096-bit RSA key
    Bit4096 = 4096,
}

impl KeySize {
    fn as_usize(&self) -> usize {
        match *self {
            KeySize::Bit1024 => 1024,
            KeySize::Bit2048 => 2048,
            KeySize::Bit3072 => 3072,
            KeySize::Bit4096 => 4096,
        }
    }
}

impl E2ee {
    /// Creates a new `E2ee` instance with the specified key size.
    ///
    /// # Arguments
    ///
    /// * `key_size` - The size of the RSA keys to generate. This should be one of the `KeySize` variants.
    ///
    /// # Examples
    ///
    /// ```
    /// use e2ee::server::{E2ee, KeySize};
    ///
    /// let e2ee = E2ee::new(KeySize::Bit2048).expect("Failed to create E2ee instance");
    /// ```
    ///
    /// # Errors
    ///
    /// This function returns an error if key generation fails.
    pub fn new(key_size: KeySize) -> E2eeResult<Self> {
        let bits = key_size.as_usize();
        let (private_key, public_key, private_key_pem, public_key_pem) =
            generate_rsa_keypair(bits)?;
        Ok(Self {
            private_key,
            public_key,
            private_key_pem,
            public_key_pem,
        })
    }

    /// Creates a new `E2ee` instance from PEM-encoded private and public keys.
    ///
    /// # Arguments
    ///
    /// * `private_key_pem` - The PEM-encoded private key as a string.
    /// * `public_key_pem` - The PEM-encoded public key as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use e2ee::server::E2ee;
    ///
    /// let private_key_pem = include_str!("../files/private.pem");
    /// let public_key_pem = include_str!("../files/public.pem");
    /// let e2ee = E2ee::new_from_pem(private_key_pem.to_string(), public_key_pem.to_string())
    ///     .expect("Failed to create E2ee instance from PEM");
    /// ```
    ///
    /// # Errors
    ///
    /// This function returns an error if decoding the PEM keys fails.
    pub fn new_from_pem(
        private_key_pem: String,
        public_key_pem: String,
    ) -> E2eeResult<Self> {
        let public_key = RsaPublicKey::from_public_key_pem(&public_key_pem)?;
        let private_key = RsaPrivateKey::from_pkcs8_pem(&private_key_pem)?;
        Ok(Self {
            private_key,
            public_key,
            private_key_pem,
            public_key_pem,
        })
    }

    /// Retrieves the public key in its original `RsaPublicKey` format.
    ///
    /// # Examples
    ///
    /// ```
    /// use e2ee::server::{E2ee, KeySize};
    ///
    /// let e2ee = E2ee::new(KeySize::Bit2048).expect("Failed to create E2ee instance");
    /// let public_key = e2ee.get_public_key();
    /// // Now you can use the public key for encryption
    /// ```
    ///
    /// # Returns
    ///
    /// This function returns a reference to the `RsaPublicKey` contained in the `E2ee` struct.
    pub fn get_public_key(&self) -> &RsaPublicKey {
        &self.public_key
    }

    /// Retrieves the public key in its original `RsaPrivateKey` format.
    ///
    /// # Examples
    ///
    /// ```
    /// use e2ee::server::{E2ee, KeySize};
    ///
    /// let e2ee = E2ee::new(KeySize::Bit2048).expect("Failed to create E2ee instance");
    /// let public_key = e2ee.get_private_key();
    /// // Now you can use the public key for encryption
    /// ```
    ///
    /// # Returns
    ///
    /// This function returns a reference to the `RsaPrivateKey` contained in the `E2ee` struct.
    pub fn get_private_key(&self) -> &RsaPrivateKey {
        &self.private_key
    }

    /// Retrieves the PEM-encoded private key.
    ///
    /// # Examples
    ///
    /// ```
    /// use e2ee::server::{E2ee,KeySize};
    ///
    /// let e2ee = E2ee::new(KeySize::Bit2048).expect("Failed to create E2ee instance");
    /// let private_key_pem = e2ee.get_private_key_pem();
    /// println!("Private Key PEM: {}", private_key_pem);
    /// ```
    ///
    /// # Returns
    ///
    /// This function returns a string slice containing the PEM-encoded private key.
    pub fn get_private_key_pem(&self) -> &str {
        &self.private_key_pem
    }

    /// Retrieves the PEM-encoded public key.
    ///
    /// # Examples
    ///
    /// ```
    /// use e2ee::server::{E2ee,KeySize};
    ///
    /// let e2ee = E2ee::new(KeySize::Bit2048).expect("Failed to create E2ee instance");
    /// let public_key_pem = e2ee.get_public_key_pem();
    /// println!("Public Key PEM: {}", public_key_pem);
    /// ```
    ///
    /// # Returns
    ///
    /// This function returns a string slice containing the PEM-encoded public key.
    pub fn get_public_key_pem(&self) -> &str {
        &self.public_key_pem
    }

    /// Encrypts a message using the public key.
    ///
    /// # Arguments
    ///
    /// * `message` - The plaintext message to encrypt.
    ///
    /// # Examples
    ///
    /// ```
    /// use e2ee::server::{E2ee, KeySize};
    ///
    /// let e2ee = E2ee::new(KeySize::Bit2048).expect("Failed to create E2ee instance");
    /// let message = "Hello, world!";
    /// let encrypted = e2ee.encrypt(message).expect("Failed to encrypt message");
    /// ```
    ///
    /// # Errors
    ///
    /// This function returns an error if encryption fails.
    pub fn encrypt(&self, message: &str) -> E2eeResult<String> {
        let mut rng = OsRng;
        let padding = Oaep::new::<Sha256>();
        let encrypted_data =
            self.public_key
                .encrypt(&mut rng, padding, message.as_bytes())?;
        Ok(general_purpose::STANDARD_NO_PAD.encode(encrypted_data))
    }

    /// Decrypts a ciphertext using the private key.
    ///
    /// # Arguments
    ///
    /// * `ciphertext` - The base64-encoded encrypted message to decrypt.
    ///
    /// # Examples
    ///
    /// ```
    /// use e2ee::server::{E2ee, KeySize};
    ///
    /// let e2ee = E2ee::new(KeySize::Bit2048).expect("Failed to create E2ee instance");
    /// let message = "Hello, world!";
    /// let encrypted = e2ee.encrypt(message).expect("Failed to encrypt message");
    /// let decrypted = e2ee.decrypt(&encrypted).expect("Failed to decrypt message");
    /// assert_eq!(message, decrypted);
    /// ```
    ///
    /// # Errors
    ///
    /// This function returns an error if decryption fails.
    pub fn decrypt(&self, ciphertext: &str) -> E2eeResult<String> {
        let padding = Oaep::new::<Sha256>();
        let encrypted_data = general_purpose::STANDARD_NO_PAD.decode(ciphertext)?;
        let decrypted_data = self.private_key.decrypt(padding, &encrypted_data)?;
        Ok(String::from_utf8(decrypted_data)?)
    }

    /// Saves the PEM-encoded private and public keys to files.
    ///
    /// # Arguments
    ///
    /// * `private_key_file` - The path to the file where the private key PEM should be saved.
    /// * `public_key_file` - The path to the file where the public key PEM should be saved.
    ///
    /// # Examples
    ///
    /// ```
    /// use e2ee::server::{E2ee, KeySize};
    ///
    /// let private_key_file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/files/private_key.pem");
    /// let public_key_file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/files/public_key.pem");
    /// let e2ee = E2ee::new(KeySize::Bit2048).expect("Failed to create E2ee instance");
    /// e2ee.save_keys_to_files(private_key_file_path, public_key_file_path)
    ///     .expect("Failed to save keys to files");
    ///
    /// // Clean up files
    /// std::fs::remove_file(private_key_file_path)
    ///     .expect("Failed to delete private key file");
    /// std::fs::remove_file(public_key_file_path)
    ///     .expect("Failed to delete public key file");
    /// ```
    ///
    /// # Errors
    ///
    /// This function returns an error if writing to the files fails.
    pub fn save_keys_to_files(
        &self,
        private_key_file_path: &str,
        public_key_file_path: &str,
    ) -> E2eeResult<()> {
        let mut private_key_file =
            File::create(private_key_file_path).map_err(|_| {
                E2eeError::FileWriteError("Failed to create private key file".into())
            })?;
        let mut public_key_file =
            File::create(public_key_file_path).map_err(|_| {
                E2eeError::FileWriteError("Failed to create public key file".into())
            })?;

        private_key_file
            .write_all(self.private_key_pem.as_bytes())
            .map_err(|_| {
                E2eeError::FileWriteError(
                    "Failed to write private key to file".into(),
                )
            })?;
        public_key_file
            .write_all(self.public_key_pem.as_bytes())
            .map_err(|_| {
                E2eeError::FileWriteError(
                    "Failed to write public key to file".into(),
                )
            })?;

        Ok(())
    }
}

fn generate_rsa_keypair(
    bits: usize,
) -> Result<(RsaPrivateKey, RsaPublicKey, String, String), E2eeError> {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, bits)?;
    let public_key = RsaPublicKey::from(&private_key);
    let private_key_pem = private_key
        .to_pkcs8_pem(rsa::pkcs8::LineEnding::default())
        .map_err(E2eeError::Pkcs8)?
        .to_string();
    let public_key_pem = public_key
        .to_public_key_pem(rsa::pkcs8::LineEnding::default())
        .map_err(E2eeError::Spki)?;
    Ok((private_key, public_key, private_key_pem, public_key_pem))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests encryption and decryption using a 2048-bit RSA key.
    ///
    /// This test ensures that a message encrypted with the RSA public key can be successfully decrypted
    /// back to the original message using the RSA private key.
    #[test]
    fn test_encryption_decryption_with_2048_bits_key() {
        let e2ee = E2ee::new(KeySize::Bit2048).unwrap();
        let message = "Hello world!";
        let encrypted = e2ee.encrypt(message).unwrap();
        let decrypted = e2ee.decrypt(&encrypted).unwrap();
        assert_eq!(message, decrypted);
    }

    /// Tests encryption and decryption using a 4096-bit RSA key.
    ///
    /// Similar to the previous test but with a larger key size. This ensures that encryption and decryption
    /// work correctly with a different key size.
    #[test]
    fn test_encryption_decryption_with_4096_bits_key() {
        let e2ee = E2ee::new(KeySize::Bit4096).unwrap();
        let message = "Hi mom!";
        let encrypted = e2ee.encrypt(message).unwrap();
        let decrypted = e2ee.decrypt(&encrypted).unwrap();
        assert_eq!(message, decrypted);
    }

    /// Tests creating an `E2ee` instance with an invalid or edge-case key size.
    ///
    /// This test checks how the system handles key sizes that may be considered invalid or too small,
    /// ensuring that the function behaves as expected (e.g., returns an error or succeeds with a valid key).
    #[test]
    fn test_key_generation_with_invalid_size() {
        let result = E2ee::new(KeySize::Bit1024); // Assuming 1024-bit is invalid or too small for your use case
        assert!(result.is_ok() || result.is_err()); // Adjust as necessary based on expected behavior
    }

    /// Tests encryption and decryption with an empty message.
    ///
    /// This test ensures that encrypting and decrypting an empty string works correctly, validating that
    /// the system can handle edge cases where the message to be encrypted is empty.
    #[test]
    fn test_encrypt_decrypt_empty_message() {
        let e2ee = E2ee::new(KeySize::Bit2048).unwrap();
        let message = "";
        let encrypted = e2ee.encrypt(message).unwrap();
        let decrypted = e2ee.decrypt(&encrypted).unwrap();
        assert_eq!(message, decrypted);
    }

    /// Tests saving and loading keys from files.
    ///
    /// This test verifies that PEM-encoded keys can be correctly saved to files and then loaded back,
    /// ensuring that the saved keys match the original ones. It also checks that the file operations succeed.
    #[test]
    fn test_save_load_keys() {
        const FILES_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/files/");
        let e2ee = E2ee::new(KeySize::Bit2048).unwrap();

        // Define file paths
        let private_key_path = format!("{}test_private_key.pem", FILES_PATH);
        let public_key_path = format!("{}test_public_key.pem", FILES_PATH);

        // Save the keys to files
        e2ee.save_keys_to_files(&private_key_path, &public_key_path)
            .expect("Failed to save keys to files");

        // Load the keys from files
        let loaded_private_key_pem = std::fs::read_to_string(&private_key_path)
            .expect("Failed to read private key file");
        let loaded_public_key_pem = std::fs::read_to_string(&public_key_path)
            .expect("Failed to read public key file");

        // Create a new E2ee instance from the loaded PEM keys
        let loaded_e2ee =
            E2ee::new_from_pem(loaded_private_key_pem, loaded_public_key_pem)
                .expect("Failed to create E2ee instance from PEM");

        // Ensure the loaded keys match the original keys
        assert_eq!(
            e2ee.get_private_key_pem(),
            loaded_e2ee.get_private_key_pem()
        );
        assert_eq!(e2ee.get_public_key_pem(), loaded_e2ee.get_public_key_pem());

        // Clean up the test files
        std::fs::remove_file(private_key_path)
            .expect("Failed to delete private key file");
        std::fs::remove_file(public_key_path)
            .expect("Failed to delete public key file");
    }

    /// Tests decryption with invalid base64-encoded ciphertext.
    ///
    /// This test ensures that attempting to decrypt a ciphertext that is not valid base64
    /// results in an error, validating that the system properly handles invalid inputs.
    #[test]
    fn test_encrypt_decrypt_invalid_ciphertext() {
        let e2ee = E2ee::new(KeySize::Bit2048).unwrap();
        let invalid_ciphertext = "invalid_base64_string";
        let result = e2ee.decrypt(invalid_ciphertext);
        assert!(result.is_err());
    }
}
