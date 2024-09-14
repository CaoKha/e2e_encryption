use e2ee::server::{E2ee, KeySize}; // Import the E2ee system and KeySize enum

fn main() {
    // Create a new E2EE instance with 2048-bit key size
    let e2ee = E2ee::new(KeySize::Bit2048).expect("Failed to initialize E2ee");

    // Encrypt a message
    let message = "This is a secret message.\nCan you handle line breaks ?\nSpecial characters @!#@$#%^$&^%% ?";
    let encrypted_message = e2ee.encrypt(message).expect("Encryption failed");
    println!("Encrypted message:\n{}", encrypted_message);

    // Decrypt the message
    let decrypted_message =
        e2ee.decrypt(&encrypted_message).expect("Decryption failed");
    println!("Decrypted message:\n{}", decrypted_message);

    // Assert that the decrypted message matches the original
    assert_eq!(message, decrypted_message);
}
