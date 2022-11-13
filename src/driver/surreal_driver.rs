use crate::connection::surreal_ws_conn::SurrealWsConnection;

enum Method {
    Ping,
}
impl Method {
    fn as_str(&self) -> &'static str {
        match self {
            Method::Ping => "ping"
        }        
    }
}

pub struct SurrealDriver {
    conn: SurrealWsConnection
}

impl SurrealDriver {
    fn new(conn: SurrealWsConnection) -> Self {
        Self {
            conn
        }
    }

    async fn ping(&mut self) {
        let method = Method::Ping.as_str();
        
        let _ = self.conn.rpc(method, Vec::new());
    }
}
