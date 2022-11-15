use serde::Serialize;
use serde::ser::SerializeStruct;
use std::mem::size_of;
use surrealdb::sql::Object;

#[derive(Serialize)]
#[serde(untagged)]
pub enum RpcParams {
    Objects(Vec<Object>),
    Array(Vec<String>)
}

#[repr(C)]
pub struct RpcRequest {
    id: String,
    method: String,
    params: RpcParams
}

#[allow(unused)]
impl RpcRequest {
    pub fn new(id: String, method: String, params: RpcParams) -> Self {
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
