use tungstenite::Error;
use tokio::sync::mpsc::Receiver;
use crate::{ router::message::RouterMessage, driver::surreal_driver::SurrealDriver };

use super::message::RouterMessageHelper;

pub(crate) struct MsgRouterActor {
    pub receiver: Receiver<RouterMessageHelper>,
}

impl MsgRouterActor {
    pub fn new(receiver: Receiver<RouterMessageHelper>) -> Self {
        MsgRouterActor { receiver }
    }

    #[allow(unused)]
    pub async fn handle_msg(&self, driver: &mut SurrealDriver, msg_helper: RouterMessageHelper) -> Result<(), Error> {
        // receive message and trigger appropriate surreal call
        match msg_helper.msg_type {
            RouterMessage::SignIn { username, password } => {
                let msg = driver.sign_in(&username, &password).await?;
                
                msg_helper.sender.send(Ok(msg));
                
                Ok(())
            },
            RouterMessage::UseNsDb { ns, db } => {
                let msg = driver.use_ns_db(&ns, &db).await?;

                msg_helper.sender.send(Ok(msg));

                Ok(())
            },
            RouterMessage::Query { query, args } => {
                let msg = driver.query(&query, args).await?;
                
                msg_helper.sender.send(Ok(msg));

                Ok(())
            },
            _ => { panic!("Unknown message type!"); }
        }
    }
}

