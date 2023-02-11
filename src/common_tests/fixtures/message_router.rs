use crate::router::message_router::MsgRouter;

pub fn create_message_router() -> MsgRouter {
    let (_, receiver) = tokio::sync::mpsc::channel(100);
    MsgRouter {
        receiver
    }
}