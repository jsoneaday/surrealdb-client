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
        let (sender, receiver) = 
            MsgRouterBuilder::setup_msg_helper_sender_receiver();
        let msg_router = MsgRouterBuilder::build_msg_router(receiver);
        let driver = MsgRouterBuilder::build_conn_driver(host, port, use_tls).await;
        MsgRouterBuilder::enable_msg_router_handler(msg_router, driver).await;

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
    pub async fn build_conn_driver(host: String, port: usize, use_tls: bool) -> SurrealDriver {
        let mut conn = SurrealWsConnection::new(host, port, use_tls);
        _ = conn.connect().await;
        SurrealDriver::new(conn)
    }

    #[allow(unused)]
    pub async fn enable_msg_router_handler(mut msg_router: MsgRouter, mut driver: SurrealDriver) {
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
    use mockall::predicate::*;
    use crate::common_tests::fixtures::message_router::create_message_router;
    use crate::common_tests::fixtures::globals::{ HOST, PORT };
    use crate::common_tests::fixtures::channels::oneshot::get_one_shot_channel;
    use crate::common_tests::fixtures::singleton_driver::create_driver;
    use tungstenite::Error;

    #[tokio::test]
    async fn test_setup_msg_helper_sender_receiver_returns_router_msg() {
        let _username: &str = "dave";
        let _password: &str = "123";
        let (msg_helper_sender, mut msg_helper_receiver) = 
            MsgRouterBuilder::setup_msg_helper_sender_receiver();
        
        tokio::spawn(async move {
            while let Some(msg) = msg_helper_receiver.recv().await {
                match msg.msg_type {
                    RouterMessage::SignIn { username, password } => {
                        assert!(_username == username);
                        assert!(_password == password);
                    },
                    _ => panic!("Wrong message type received")
                }
            }
        });

        let (oneshot_sender, _) = get_one_shot_channel::<Result<Message, Error>>();
        _ = msg_helper_sender.send(RouterMessageHelper { 
            sender: oneshot_sender, 
            msg_type: RouterMessage::SignIn { username: _username.to_string(), password: _password.to_string() } 
        });
    }

    #[tokio::test]
    async fn test_build_msg_router_created_without_panic() {
        let (_, receiver) = tokio::sync::mpsc::channel(100);
        MsgRouterBuilder::build_msg_router(receiver);
    }

    #[tokio::test]
    async fn test_build_conn_driver_created_without_panic() {
        MsgRouterBuilder::build_conn_driver(HOST.to_string(), PORT, false).await;
    }

    #[tokio::test]
    async fn test_enable_msg_router_handler() {
        let msg_router = create_message_router();
        let driver = create_driver().await;
        MsgRouterBuilder::enable_msg_router_handler(msg_router, driver).await;
    }
}