use tokio::sync::mpsc::Receiver;
use crate::{router::message::RouterMessage, driver::surreal_driver::{SurrealDriver, TungsteniteResult}};

use super::message::RouterMessageHelper;

pub(crate) struct MsgRouterActor {
    pub receiver: Receiver<RouterMessageHelper>,
}

impl MsgRouterActor {
    pub fn new(receiver: Receiver<RouterMessageHelper>) -> Self {
        MsgRouterActor { receiver }
    }

    #[allow(unused)]
    pub async fn handle_msg(&self, driver: &mut SurrealDriver, msg_helper: RouterMessageHelper) -> TungsteniteResult {
        // receive message and trigger appropriate surreal call
        match msg_helper.msg_type {
            RouterMessage::SignIn {  username, password } => {
                todo!()
            },
            RouterMessage::UseNsDb { ns, db } => {
                todo!()
            },
            RouterMessage::Query { query_str, args } => {
                todo!()
            },
            _ => { panic!("Unknown message type!"); }
        }
    }
}

