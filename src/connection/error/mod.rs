use thiserror::Error;

#[derive(Debug, Error)]
pub enum SurrealError {
    #[error("Surreal Authentication Error occurred")]
    SurrealAuthenticationError,

    #[error("Surreal Failed To Connect Error occurred")]
    SurrealFailedToConnectError,

    #[error("Surreal Connection Timeout Error occurred")]
    SurrealConnectionTimeoutError,

    #[error("Surreal No Database Selected Error occurred")]
    SurrealNoDatabaseSelectedError,

    #[error("Surreal Not Connected Error occurred")]
    SurrealNotConnectedError,

    #[error("Surreal Record Already Exists Error occurred")]
    SurrealRecordAlreadyExistsError
}