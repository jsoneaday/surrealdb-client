use tokio::net::TcpStream;
use tokio_tungstenite::*;
use url::Url;
use std::collections::BTreeMap;
use std::time::Duration;
use super::surreal_connection::SurrealConnection;
use async_trait::async_trait;
use futures_util::{ StreamExt };

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
    async fn connect(&mut self, timeout: Duration) {    
        let (mut ws_socket, _) = connect_async(
            Url::parse(format!("{}{}:{}/rpc", if self.use_tls == true { "wss://" } else { "ws://" }, self.host, self.port).as_str()).expect("")
        )
        .await
        .unwrap();

        self.socket = Some(ws_socket);
    }
}