use thiserror::Error;

#[derive(Debug, Error)]
#[error("Surreal Error occurred")]
pub struct SurrealError;

#[derive(Debug, Error)]
#[error("Surreal Authentication Error occurred")]
pub struct SurrealAuthenticationError;

#[derive(Debug, Error)]
#[error("Surreal Connection Timeout Error occurred")]
pub struct SurrealConnectionTimeoutError;

#[derive(Debug, Error)]
#[error("Surreal No Database Selected Error occurred")]
pub struct SurrealNoDatabaseSelectedError;

#[derive(Debug, Error)]
#[error("Surreal Not Connected Error occurred")]
pub struct SurrealNotConnectedError;

#[derive(Debug, Error)]
#[error("Surreal Record Already Exists Error occurred")]
pub struct SurrealRecordAlreadyExistsError;