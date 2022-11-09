use std::borrow::{Borrow, BorrowMut};

#[allow(unused)]
use futures_util::{SinkExt, StreamExt};
use futures_util::stream::{SplitSink, SplitStream};
use tokio::net::{TcpStream, TcpListener};
use tungstenite::{Message, Result};
use tokio_tungstenite::{accept_async, WebSocketStream};
//use std::collections::BTreeMap;
use super::{error::SurrealError};

#[allow(unused)]
pub struct SurrealWsConnection {
    use_tls: bool,
    host: &'static str,
    port: usize,
    writer: Option<SplitSink<WebSocketStream<TcpStream>, Message>>,
    reader: Option<SplitStream<WebSocketStream<TcpStream>>>
}

impl SurrealWsConnection {
    pub fn new(host: &'static str, port: usize, use_tls: bool) -> Self {
        SurrealWsConnection {
            use_tls,
            host,
            port,
            writer: None,
            reader: None
        }
    }

    async fn connect(&mut self) -> Result<(), SurrealError> {     
        println!("Start connect");    

        let immut_self = &*self; 
        let try_listener = TcpListener::bind(format!(
            "{}{}:{}/rpc", 
            if immut_self.use_tls == true { "wss://" } else { "ws://" }, 
            immut_self.host, 
            immut_self.port
        ).as_str()).await;
        if let Some(err) = try_listener.as_ref().err() {
            println!("Failed to connect: {:?}", err);
            return Err(SurrealError::SurrealFailedToConnectError);
        }
        let listener = try_listener.unwrap();

        while let Ok((stream, _)) = listener.accept().await {
            let conn_result = accept_async(stream).await;

            if let Some(err) = conn_result.as_ref().err() {
                println!("Failure: {:?}", err);
                return Err(SurrealError::SurrealFailedToConnectError);
            };
            
            let ws_socket = conn_result.unwrap();         
            let (writer, reader) = ws_socket.split();
            self.writer = Some(writer);    
            self.reader = Some(reader);        
        }

        Ok(())
    }

    async fn disconnect(&mut self) {
        let _ = self.writer.as_mut().unwrap().close();
    }

    async fn exec<T>(&mut self) {
        match &mut self.writer {
            Some(writer) => {
                let _ = writer.send(Message::Text("test string".to_owned())).await;
            },
            None => println!("socket is empty")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HOST: &str = "localhost";
    const PORT: usize = 8000;

    #[tokio::test]
    async fn int_verify_connect_makes_connection_to_surrealdb() {
        let mut conn = SurrealWsConnection::new(HOST, PORT, false);
        let result = conn.connect().await;
        
        assert!(result.is_ok());
        let _ = conn.socket.unwrap().close(None).await;
    }

    #[tokio::test]
    async fn int_verify_disconnect_is_disconnecting() {
        todo!("needs test, but after some of the actual queries are built out");
    }
}