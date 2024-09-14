use clap::Parser;
use e2ee::server::{E2ee, KeySize};

/// Simple CLI tool to generate and save RSA keys to files
#[derive(Parser, Debug)]
#[command(
    name = "E2E encryption",
    version = env!("E2EE_LIB_VERSION"),
    about = "Generate and save keys to files using RSA encryption"
)]
struct CliArgs {
    /// The size of the RSA key (1024, 2048, 3072, 4096)
    #[arg(
        short = 's',
        long = "key-size",
        value_enum,
        default_value = "bit2048",
        help = "The size of the RSA key. Possible values: bit1024, bit2048, bit3072, bit4096. Defaults to bit2048."
    )]
    key_size: KeySize,
}

fn main() {
    const FILES_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/files/");
    let args = CliArgs::parse();

    // Create E2EE instance
    let e2ee = E2ee::new(args.key_size).expect("Failed to create E2EE instance");

    // Save public and private keys to files
    e2ee.save_keys_to_files(
        format!("{}private.pem", FILES_PATH).as_str(),
        format!("{}public.pem", FILES_PATH).as_str(),
    )
    .expect("Failed to save keys to files");

    // Output the encrypted message
    println!("Public pem:\n{}\n", e2ee.get_public_key_pem());
    println!("Private pem:\n{}", e2ee.get_private_key_pem());
}
