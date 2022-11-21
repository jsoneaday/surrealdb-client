use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SurrealResult<T> {
    pub result: T,
    pub status: String,
    pub time: String
}