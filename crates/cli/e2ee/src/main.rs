use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use e2ee::{
    client::PublicE2ee,
    server::{E2ee, KeySize},
};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "E2E encryption CLI",
    version = env!("E2EE_CLI_VERSION"),
    about = "CLI tool to encrypt and decrypt messages using RSA encryption"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new pair of RSA keys and save them to files
    GenerateKeys {
        #[arg(
            short = 's',
            long = "size",
            default_value = "bit2048",
            help = "Key size"
        )]
        key_size: KeySize,
        #[arg(
            long = "public-key-file-path",
            default_value = "public.pem",
            help = "Path to public key file"
        )]
        public_key_file_path: PathBuf,
        #[arg(
            long = "private-key-file-path",
            default_value = "private.pem",
            help = "Path to private key file"
        )]
        private_key_file_path: PathBuf,
    },

    /// Encrypt a message using a public RSA key
    Encrypt {
        #[arg(short, long)]
        public_key_file_path: PathBuf,
        #[arg(short, long)]
        message: String,
    },

    /// Decrypt a ciphertext using a private RSA key
    Decrypt {
        #[arg(long, default_value = "private.pem")]
        private_key_file_path: PathBuf,
        #[arg(short, long, default_value = "public.pem")]
        public_key_file_path: PathBuf,
        #[arg(short, long)]
        ciphertext: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::GenerateKeys {
            key_size,
            public_key_file_path,
            private_key_file_path,
        } => {
            let e2ee_server =
                E2ee::new(*key_size).context("Failed to create SDK")?;
            println!("Public Key Pem:\n{}", e2ee_server.get_public_key_pem());
            println!("Private Key Pem:\n{}", e2ee_server.get_private_key_pem());
            e2ee_server
                .save_keys_to_files(
                    private_key_file_path.to_str().unwrap(),
                    public_key_file_path.to_str().unwrap(),
                )
                .context("Failed to save keys to files")?;
            println!(
                "Public Key Pem is saved to: {}",
                public_key_file_path.display()
            );
            println!(
                "Private Key Pem is saved to: {}",
                private_key_file_path.display()
            );
        }
        Commands::Encrypt {
            public_key_file_path,
            message,
        } => {
            let public_key_pem = std::fs::read_to_string(public_key_file_path)
                .context("Failed to read public key file")?;
            let e2ee_client = PublicE2ee::new(public_key_pem)?;
            let encrypted = e2ee_client
                .encrypt(message)
                .context("Failed to encrypt message")?;
            println!("Encrypted message: {}", encrypted);
        }
        Commands::Decrypt {
            private_key_file_path,
            public_key_file_path,
            ciphertext,
        } => {
            let private_key_pem = std::fs::read_to_string(private_key_file_path)
                .context("Failed to read private key file")?;
            let public_key_pem = std::fs::read_to_string(public_key_file_path)
                .context("Failed to read public key file")?;
            let e2ee_server = E2ee::new_from_pem(private_key_pem, public_key_pem)
                .context("Failed to create SDK")?;
            let decrypted = e2ee_server
                .decrypt(ciphertext)
                .context("Failed to decrypt message")?;
            println!("Decrypted message: {}", decrypted);
        }
    }

    Ok(())
}
