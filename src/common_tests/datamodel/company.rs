use tungstenite::{Message, Error};
use serde::Deserialize;
use crate::connection::model::rpcresponse::RpcResponse;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Company {
    pub id: String,
    pub name: String
}

impl Company {
    pub fn from_result_message(msg: &Result<Message, Error>) -> Option<RpcResponse<Company>> {
        let Ok(message) = msg else {
            return None;
        };

        let company_resp: Result<RpcResponse<Company>, serde_json::Error> = match message {
            Message::Text(json_str) => {
                serde_json::from_str(json_str)
            },
            _ => serde_json::from_str("")
        };
        let Ok(co_resp) = company_resp else {
            return None;
        };

        Some(co_resp)
    }

    pub fn get_first<'a>(co_response: &'a RpcResponse<Company>) -> Option<&'a Company> {
        let Some(surreal_result) = co_response.result[0].result.as_ref() else {
            return None;
        };
        Some(&surreal_result[0])
    }
}