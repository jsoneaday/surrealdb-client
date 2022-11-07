use tokio::net::{TcpStream};
use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream};
use url::Url;
//use std::collections::BTreeMap;
use super::{surreal_connection::SurrealConnection, error::SurrealError};
use async_trait::async_trait;

pub struct SurrealWsConnection {
    use_tls: bool,
    host: &'static str,
    port: usize,
    socket: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>
}

impl SurrealWsConnection {
    pub fn new(host: &'static str, port: usize, use_tls: bool) -> Self {
        SurrealWsConnection {
            use_tls,
            host,
            port,
            socket: None
        }
    }
}

#[async_trait]
impl SurrealConnection for SurrealWsConnection {
    async fn connect(&mut self) -> Result<(), SurrealError> {     
        println!("Start connect");    

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
        self.socket = Some(ws_socket);

        Ok(())        
    }

    async fn disconnect(&mut self) {
        let _ = self.socket.as_mut().unwrap().close(None);
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