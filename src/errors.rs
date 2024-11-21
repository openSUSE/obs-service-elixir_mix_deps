use thiserror::Error;

#[derive(Error, Debug)]
pub enum SourceServiceError {
    #[error("I/O error")]
    IO(#[from] std::io::Error),
}
