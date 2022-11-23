use serde::{Deserialize};
use super::surrealresult::SurrealResult;

#[derive(Debug, Deserialize)]
pub struct RpcResponse<T> {
    pub id: String,
    pub result: Vec<SurrealResult<Vec<T>>>    
}