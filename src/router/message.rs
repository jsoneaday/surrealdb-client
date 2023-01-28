use std::collections::BTreeMap;
use tungstenite::Message;
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum RouterMessage {
    SignIn {
        username: String,
        password: String
    },
    UseNsDb {
        ns: String,
        db: String
    },
    Query {        
        query_str: String,
        args: BTreeMap<String, String>
    }
}

#[allow(unused)]
#[derive(Debug)]
pub(crate) struct RouterMessageHelper {
    pub sender: oneshot::Sender<Message>,
    pub msg_type: RouterMessage,
}

#[derive(Debug, Clone)]
pub struct RouterMessageError(pub String);