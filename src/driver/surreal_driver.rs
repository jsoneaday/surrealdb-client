use crate::connection::surreal_ws_conn::{SurrealWsConnection, Method};

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
        let result = self.conn.rpc(Method::Ping, Vec::new());
        println!("{:?}", result.await);
    }
}
