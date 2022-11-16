use serde::{Deserialize};
use super::result::Result;

#[derive(Debug, Deserialize)]
pub struct RpcResponse<T> {
    id: String,
    result: Vec<Result<Vec<T>>>    
}