use thiserror::Error;

#[derive(Debug, Error)]
enum NetworkError {
    #[error("connection timed out")]
    Timeout,
}

#[derive(Debug, Error)]
enum DatabaseError {
    #[error("error querying database")]
    QueryFailure,
}
