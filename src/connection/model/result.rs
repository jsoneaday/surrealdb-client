use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Result<T> {
    pub result: T,
    pub status: String,
    pub time: String
}