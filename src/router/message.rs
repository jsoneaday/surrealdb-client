use tungstenite::Message;

#[derive(Debug)]
pub struct RouterMessage {
    pub message: Message
}