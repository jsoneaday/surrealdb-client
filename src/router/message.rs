use std::collections::BTreeMap;
use tungstenite::{ Error as TungsteniteError, Message };
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::RecvError;

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
        query: String,
        args: BTreeMap<String, String>
    }
}

#[allow(unused)]
#[derive(Debug)]
pub(crate) struct RouterMessageHelper {
    pub sender: oneshot::Sender<Result<Message, TungsteniteError>>,
    pub msg_type: RouterMessage,
}

#[derive(Debug)]
pub enum RouterMessageError {
    Tungstenite(TungsteniteError),
    ReceiveError(RecvError)
}