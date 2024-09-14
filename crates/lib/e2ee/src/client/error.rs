use thiserror::Error;
pub type PublicE2eeResult<T> = std::result::Result<T, PublicE2eeError>;

#[derive(Error, Debug)]
pub enum PublicE2eeError {
    #[error("RSA error: {0}")]
    Rsa(#[from] rsa::errors::Error),

    #[error("PKCS#8 error: {0}")]
    Pkcs8(#[from] rsa::pkcs8::Error),

    #[error("SPKI error: {0}")]
    Spki(#[from] rsa::pkcs8::spki::Error),

    #[error("Encoding error: {0}")]
    Encoding(#[from] std::string::FromUtf8Error),

    #[error("Decoding error: {0}")]
    Decoding(#[from] base64::DecodeError),

    // #[error("File write error: {0}")]
    // FileWriteError(String),
}
