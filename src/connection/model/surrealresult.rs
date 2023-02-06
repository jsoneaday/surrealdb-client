use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SurrealResult<T> {
    pub detail: Option<String>,
    pub result: Option<T>,
    pub status: String,
    pub time: String
}


pub const SURREALRESULT_STATUS_OK: &str = "OK";
pub const SURREALRESULT_STATUS_ERR: &str = "ERR";