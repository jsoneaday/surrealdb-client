#[allow(unused)]
use futures_util::{SinkExt, StreamExt, future, pin_mut};
use futures_util::stream::{SplitSink, SplitStream};
use tokio::net::TcpStream;
use tungstenite::{Message, Result, Error};
use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream};
use super::{error::SurrealError, model::rpcrequest::{RpcRequest, RpcParams}};
use super::model::method::Method;
use url::Url;
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::RwLock;


pub struct SurrealWsConnection {
    last_request_id: Arc<RwLock<Uuid>>,
    use_tls: bool,
    host: String,
    port: usize,
    writer: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
    reader: Option<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>
}

impl SurrealWsConnection {
    pub fn new(host: String, port: usize, use_tls: bool) -> Self {
        SurrealWsConnection {
            last_request_id: Arc::new(RwLock::new(Uuid::new_v4())),
            use_tls,
            host,
            port,
            writer: None,
            reader: None
        }
    }

    pub async fn connect(&mut self) -> Result<(), SurrealError> {   
        let conn_result = connect_async(
            Url::parse(format!("{}{}:{}/rpc", if self.use_tls == true { "wss://" } else { "ws://" }, 
            self.host, self.port).as_str()).expect("")
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

    pub async fn disconnect(&mut self) -> Result<(), Error> {
        self.writer.as_mut().unwrap().close().await
    }

    pub async fn rpc(&mut self, method: Method, params: RpcParams) -> Result<Message, Error> {
        let meth = method.as_str();
        let mut last_request_id = self.last_request_id.write().await;
        *last_request_id = Uuid::new_v4();
        let rpc_req: RpcRequest = RpcRequest::new(last_request_id.to_string(), meth.to_string(), params);
        let json = serde_json::to_string(&rpc_req);
        let json_txt = json.unwrap();
                
        match (&mut self.writer, &mut self.reader) {
            (Some(writer), Some(reader)) => {
                writer.send(Message::Text(json_txt)).await?;
                
                loop {
                    tokio::select! {
                        opt_msg = reader.next() => {
                            match opt_msg {
                                Some(result_msg) => {
                                    return result_msg;
                                },
                                None => {
                                    break;
                                }
                            }
                        }
                    }
                }
            },
            _ => {}
        }        

        Ok(Message::Text("invalid nothing returned".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HOST: &str = "localhost";
    const PORT: usize = 8000;

    async fn get_conn() -> SurrealWsConnection {
        let mut surreal_conn = SurrealWsConnection::new(HOST.to_string(), PORT, false);
        let _ = surreal_conn.connect().await;
        surreal_conn
    }

    #[tokio::test]
    async fn connection_succeeds() {
        let mut surreal_conn = SurrealWsConnection::new(HOST.to_string(), PORT, false);
        let result = surreal_conn.connect().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn surreal_conn_disconnect_succeeds() {
        let mut surreal_conn = get_conn().await;

        let result = surreal_conn.disconnect().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn surreal_conn_rpc_succeeds() {
        let mut surreal_conn = get_conn().await;

        let result = surreal_conn.rpc(Method::Ping, RpcParams::Array(Vec::new())).await;

        assert!(result.is_ok());
    }
}