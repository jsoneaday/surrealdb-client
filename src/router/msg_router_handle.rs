use tokio::sync::mpsc::Sender;
use tungstenite::Message;
use super::{message::RouterMessage, message_router::{run_router, MsgRouterActor}};
use std::error::Error;

pub struct MsgRouterHandle {
    sender: Sender<RouterMessage>
}

impl MsgRouterHandle {
    pub fn new(host: String, port: usize, use_tls: bool) -> Self {
        let (sender, receiver) = tokio::sync::mpsc::channel(100);
        let mut router = MsgRouterActor::new(receiver);
        tokio::spawn(run_router(router, host, port, use_tls));

        Self { sender }
    }

    pub async fn send_message(&self, message: Message) -> Result<(), Box<dyn Error>> {
        let msg = RouterMessage {
            message
        };
        self.sender.send(msg).await?;
        Ok(())
    }
}