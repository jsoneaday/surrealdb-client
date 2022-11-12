use serde::Serialize;
use serde::ser::SerializeStruct;
use surrealdb::sql::Object;
use std::mem::size_of;

#[repr(C)]
pub struct RpcRequest {
    id: String,
    method: String,
    params: Vec<Object>
}

#[allow(unused)]
impl RpcRequest {
    pub fn new(id: String, method: String, params: Vec<Object>) -> Self {
        RpcRequest { 
            id, 
            method,
            params
        }
    }
}

impl Serialize for RpcRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        
        let mut state = serializer.serialize_struct("RpcRequest", size_of::<RpcRequest>())?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("method", &self.method)?;
        state.serialize_field("params", &self.params)?;
        state.end()
    }
}