use serde::{Deserialize, de::DeserializeOwned};
use tungstenite::Message;
use super::surrealresult::SurrealResult;

#[derive(Debug, Deserialize)]
pub struct RpcResponse<T> {
    pub id: String,
    pub result: Vec<SurrealResult<Vec<T>>>    
}

impl<T> RpcResponse<T>
where
    T: DeserializeOwned {
    pub fn deserialize(message: &Message) -> Result<RpcResponse<T>, serde_json::Error> {
        match message {
            Message::Text(txt) => {
                println!("txt to deserialize: {:?}", txt);
                serde_json::from_str(txt.as_str())
            },
            _ => {
                serde_json::from_str("")
            }
        }
    }
}