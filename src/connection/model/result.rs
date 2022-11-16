use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Result<T> {
    result: T,
    status: String,
    time: String
}