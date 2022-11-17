use serde::{Deserialize};
use super::result::Result;

#[derive(Debug, Deserialize)]
pub struct RpcResponse<T> {
    pub id: String,
    pub result: Vec<Result<Vec<T>>>    
}