use std::collections::BTreeMap;
use tungstenite::{ Error as TungsteniteError, Message };
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::RecvError;
use tokio;

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
pub struct RouterMessageHelper {
    pub sender: oneshot::Sender<Result<Message, TungsteniteError>>,
    pub msg_type: RouterMessage,
}

#[derive(Debug)]
pub enum RouterMessageError {
    Tungstenite(TungsteniteError),
    ReceiveError(RecvError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn routermessagehelper_should_send_message() {
        let (sender, receiver) = oneshot::channel();
        let helper = RouterMessageHelper {
            sender,
            msg_type: RouterMessage::SignIn { username: "root".to_string(), password: "root".to_string() }
        };

        let msg_to_send = Message::Text("test string".to_string());
        let cloned_msg = msg_to_send.clone();
        println!("testing");
        let join = tokio::spawn(async move {            
            match receiver.await {
                Ok(result) => {
                    match result {
                        Ok(msg) => {
                            msg
                        },
                        Err(_) => panic!("Error receiving")
                    }
                },
                Err(_) => panic!("Error receiving")
            }
        });
        
        let cloned_again_msg = cloned_msg.clone();
        _ = helper.sender.send(Ok(cloned_msg));
        let joined_result = join.await;
        assert_eq!(joined_result.unwrap(), cloned_again_msg.clone());        
    }
}