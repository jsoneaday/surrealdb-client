use tokio::sync::mpsc::Receiver;
use crate::{router::message::RouterMessage, driver::surreal_driver::SurrealDriver, connection::surreal_ws_conn::SurrealWsConnection};

pub struct MsgRouterActor{
    pub receiver: Receiver<RouterMessage>,
}

impl MsgRouterActor {
    pub fn new(receiver: Receiver<RouterMessage>) -> Self {
        MsgRouterActor { receiver }
    }

    pub async fn handle_msg(&self, driver: &mut SurrealDriver, msg: RouterMessage) {
        // receive message and trigger appropriate surreal call
        match msg {
            RouterMessage::SignIn {  } => { },
            RouterMessage::UseNsDb {  } => { },
            RouterMessage::Query {  } => { },
            _ => { panic!("Unknown message type!"); }
        }
    }
}

pub async fn run_router(mut router: MsgRouterActor, host: String, port: usize, use_tls: bool) {
    let mut conn = SurrealWsConnection::new(host, port, use_tls);
    let _ = conn.connect().await;
    let mut driver = SurrealDriver::new(conn);
    
    while let Some(msg) = router.receiver.recv().await {
        router.handle_msg(&mut driver, msg).await
    }
}