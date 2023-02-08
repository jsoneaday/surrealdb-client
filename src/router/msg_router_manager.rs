use tokio::sync::mpsc::{ Sender, Receiver };
use tokio::sync::oneshot;
use super::message::{RouterMessageHelper, RouterMessageError};
use super::{message::RouterMessage, message_router::MsgRouter};
use crate::connection::surreal_ws_conn::SurrealWsConnection;
use crate::driver::surreal_driver::SurrealDriver;
use tungstenite::{ Message };

pub struct MsgRouterManager {
    sender: Sender<RouterMessageHelper>
}

impl MsgRouterManager {
    pub async fn build_msg_router_manager(host: String, port: usize, use_tls: bool) -> Self {
        let sender = MsgRouterManager::build_msg_router(host, port, use_tls).await;
        Self { sender }
    }

    async fn build_msg_router(host: String, port: usize, use_tls: bool) -> Sender<RouterMessageHelper> {
        let (sender, receiver) = MsgRouterBuilder::setup_msg_helper_sender_receiver();
        let msg_router = MsgRouterBuilder::build_msg_router(receiver);
        MsgRouterBuilder::enable_msg_router_receiver(msg_router, host, port, use_tls).await;

        sender
    }

    pub async fn send_msg_to_msg_router_and_wait_receive(&self, msg: RouterMessage) -> Result<Message, RouterMessageError> {
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

    
}

pub struct MsgRouterBuilder;

impl MsgRouterBuilder {
    fn setup_msg_helper_sender_receiver() -> (Sender<RouterMessageHelper>, Receiver<RouterMessageHelper>) {
        return tokio::sync::mpsc::channel::<RouterMessageHelper>(100);
    }

    #[allow(unused)]
    pub fn build_msg_router(receiver: Receiver<RouterMessageHelper>) -> MsgRouter {
        MsgRouter { receiver }
    }

    #[allow(unused)]
    pub async fn enable_msg_router_receiver(mut msg_router: MsgRouter, host: String, port: usize, use_tls: bool) {
        let mut conn = SurrealWsConnection::new(host, port, use_tls);
        _ = conn.connect().await;
        let mut driver = SurrealDriver::new(conn);
        
        tokio::spawn(async move {
            while let Some(msg) = msg_router.receiver.recv().await {
                _ = msg_router.handle_msg(&mut driver, msg).await.unwrap();
            }
        });        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn check_setup_msg_helper_sender_receiver_returns_objects() {
        let _username: &str = "dave";
        let _password: &str = "123";
        let (msg_helper_sender, mut msg_helper_receiver) = MsgRouterBuilder::setup_msg_helper_sender_receiver();
        
        tokio::spawn(async move {
            while let Some(msg) = msg_helper_receiver.recv().await {
                match msg.msg_type {
                    RouterMessage::SignIn { username, password } => {
                        if _username != username || _password != password {
                            panic!("RouterMessage fields do not match");
                        }
                    },
                    _ => panic!("Wrong message type received")
                }
            }
        });

        let (msg_sender, _) = tokio::sync::oneshot::channel();
        _ = msg_helper_sender.send(RouterMessageHelper { 
            sender: msg_sender, 
            msg_type: RouterMessage::SignIn { username: _username.to_string(), password: _password.to_string() } 
        });
    }
}