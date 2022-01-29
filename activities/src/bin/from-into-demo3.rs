use thiserror::Error;

#[derive(Debug, Error)]
enum NetworkError {
    #[error("connection timed out")]
    Timeout,
}
