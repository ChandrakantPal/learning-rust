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

#[derive(Debug, Error)]
enum ApiError {
    #[error("network error: {0}")]
    Network(NetworkError),
    #[error("database error: {0}")]
    Database(DatabaseError),
}

impl From<NetworkError> for ApiError {
    fn from(err: NetworkError) -> Self {
        Self::Network(err)
    }
}

fn do_stuff() -> Result<(), ApiError> {
    Err(NetworkError::Timeout)?
}
