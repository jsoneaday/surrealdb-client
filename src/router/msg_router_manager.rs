use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use super::message::{RouterMessageHelper, RouterMessageError};
use super::{message::RouterMessage, message_router::MsgRouterActor};
use crate::connection::surreal_ws_conn::SurrealWsConnection;
use crate::driver::surreal_driver::SurrealDriver;
use tungstenite::{ Message };

pub struct MsgRouterManager {
    sender: Sender<RouterMessageHelper>
}

impl MsgRouterManager {
    pub fn new(host: String, port: usize, use_tls: bool) -> Self {
        let (sender, receiver) = tokio::sync::mpsc::channel(100);
        let router = MsgRouterActor::new(receiver);

        tokio::spawn(Self::run_router(router, host, port, use_tls));

        Self { sender }
    }

    pub async fn send_message(&self, msg: RouterMessage) -> Result<Message, RouterMessageError> {
        let (sender, receiver) = oneshot::channel();
        let msg_helper = RouterMessageHelper {
            sender,
            msg_type: msg
        };

        _ = self.sender.send(msg_helper).await;
        
        match receiver.await {
            Ok(result) => {
                match result {
                    Ok(msg) => Ok(msg),
                    Err(err) => Err(RouterMessageError::Tungstenite(err))
                }
            },
            Err(err) => {
                Err(RouterMessageError::ReceiveError(err))
            }
        }
    }

    async fn run_router(mut router: MsgRouterActor, host: String, port: usize, use_tls: bool) {
        let mut conn = SurrealWsConnection::new(host, port, use_tls);
        let _ = conn.connect().await;
        let mut driver = SurrealDriver::new(conn);
        
        while let Some(msg) = router.receiver.recv().await {
            let _ = router.handle_msg(&mut driver, msg).await.unwrap();
        }
    }
}