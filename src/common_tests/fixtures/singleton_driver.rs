use crate::{connection::surreal_ws_conn::SurrealWsConnection, driver::surreal_driver::SurrealDriver};
use crate::common_tests::fixtures::globals::{ HOST, PORT };

pub async fn create_driver() -> SurrealDriver {
    let mut conn = SurrealWsConnection::new(HOST.to_string(), PORT, false);
    _ = conn.connect().await;
    SurrealDriver::new(conn)
}