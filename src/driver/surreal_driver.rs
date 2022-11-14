use crate::connection::surreal_ws_conn::{SurrealWsConnection, Method};

#[allow(unused)]
pub struct SurrealDriver {
    conn: SurrealWsConnection
}

#[allow(unused)]
impl SurrealDriver {
    fn new(conn: SurrealWsConnection) -> Self {
        Self {
            conn
        }
    }

    async fn ping(&mut self) -> Result<tungstenite::Message, tungstenite::Error> {        
        self.conn.rpc(Method::Ping, Vec::new()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HOST: &str = "localhost";
    const PORT: usize = 8000;

    #[tokio::test]
    async fn ping_completes_successfully() {
        let surreal_conn = SurrealWsConnection::new(&HOST, PORT, false);
        let mut driver = SurrealDriver::new(surreal_conn);

        let result = driver.ping().await;
        assert!(result.is_ok());
    }
}