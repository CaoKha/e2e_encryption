use thiserror::Error;
pub type E2eeCliResult<T> = std::result::Result<T, E2eeCliError>;

#[derive(Error, Debug)]
pub enum E2eeCliError {
    #[error("E2EE error: {0}")]
    E2ee(#[from] e2ee::E2eeError),
    #[error("Initialization error: {0}")]
    Initialization(String),
}
