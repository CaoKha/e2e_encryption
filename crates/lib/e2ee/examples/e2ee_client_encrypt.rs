use clap::Parser;
use e2ee::client::PublicE2ee;

const PUBLIC_KEY_PATH: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/files/public.pem");

/// Simple CLI tool to encrypt a message using RSA
#[derive(Parser, Debug)]
#[command(
    name = "E2E encryption",
    version = env!("E2EE_LIB_VERSION"),
    about = "Encrypts a message client side using RSA encryption"
)]
struct CliArgs {
    /// The message to encrypt
    #[arg(
        short = 'm',
        long = "message",
        required = true,
        help = "The message to encrypt."
    )]
    message: String,

    /// The size of the RSA key (1024, 2048, 3072, 4096)
    #[arg(
        short = 'p',
        long = "public-key-file-path",
        value_enum,
        default_value = PUBLIC_KEY_PATH,
        help = "Path to the public key pem file."
    )]
    public_key_file_path: String,
}

fn main() {
    let args = CliArgs::parse();

    println!("Public pem location:\n{}\n", &args.public_key_file_path);

    // Read public key pem file
    let public_key_pem = std::fs::read_to_string(&args.public_key_file_path)
        .expect("Failed to read public key pem file");

    // Create E2EE instance
    let e2ee =
        PublicE2ee::new(public_key_pem).expect("Failed to create E2EE instance");

    // Encrypt the message
    let encrypted = e2ee
        .encrypt(&args.message)
        .expect("Failed to encrypt message");

    println!("Encrypted message:\n{}\n", encrypted);
}
