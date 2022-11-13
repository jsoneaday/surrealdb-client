//use std::borrow::{Borrow, BorrowMut};
//use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[allow(unused)]
use futures_util::{SinkExt, StreamExt, future, pin_mut};
use futures_util::stream::{SplitSink, SplitStream};
use surrealdb::sql::Object;
use tokio::net::{TcpStream};
use tungstenite::{Message, Result, Error};
use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream};
//use std::collections::BTreeMap;
use super::{error::SurrealError, model::rpcrequest::RpcRequest};
use url::Url;
use std::sync::atomic::AtomicU64;

#[allow(unused)]
pub struct SurrealWsConnection {
    last_request_id: AtomicU64,
    use_tls: bool,
    host: &'static str,
    port: usize,
    writer: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
    reader: Option<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>
}

impl SurrealWsConnection {
    pub fn new(host: &'static str, port: usize, use_tls: bool) -> Self {
        SurrealWsConnection {
            last_request_id: AtomicU64::default(),
            use_tls,
            host,
            port,
            writer: None,
            reader: None
        }
    }

    pub async fn connect(&mut self) -> Result<(), SurrealError> {    
        let immut_self = &*self; 
        let conn_result = connect_async(
            Url::parse(format!("{}{}:{}/rpc", if immut_self.use_tls == true { "wss://" } else { "ws://" }, immut_self.host, immut_self.port).as_str()).expect("")
        )
        .await;
        if let Some(err) = conn_result.as_ref().err() {
            println!("Failure: {:?}", err);
            return Err(SurrealError::SurrealFailedToConnectError);
        };
        
        let (ws_socket, _) = conn_result.unwrap();
        let (writer, reader) = ws_socket.split();
        self.writer = Some(writer);    
        self.reader = Some(reader);  

        Ok(())
    }

    pub async fn disconnect(&mut self) {
        let _ = self.writer.as_mut().unwrap().close();
    }

    pub async fn rpc(&mut self, method: &str, params: Vec<Object>) -> Result<(), Error> {
        let rpc_req: RpcRequest = RpcRequest::new(self.last_request_id.get_mut().to_string(), method.to_owned(), params);

        let json = serde_json::to_string(&rpc_req);

        let _ = self.writer.as_mut().unwrap().send(Message::Text(json.unwrap()));

        loop {
            tokio::select! {
                msg = self.reader.as_mut().unwrap().next() => {
                    match msg {
                        Some(msg) => {
                            println!("received {:?}", msg);
                            break;
                        },
                        None => {
                            break;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn exec(&mut self) -> Result<(), Error> {
        match (&mut self.writer, &mut self.reader) {
            (Some(writer), Some(reader)) => {
                println!("start writer.send");

                writer.send(Message::Text("test string".to_owned())).await?;

                loop {
                    tokio::select! {
                        msg = reader.next() => {
                            match msg {
                                Some(msg) => {
                                    println!("reader received {:?}", msg);
                                    break;
                                },
                                None => {
                                    break;
                                }
                            }
                        }
                    }
                }

                Ok(())
            },
            _ => {
                println!("socket is empty");
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    
}