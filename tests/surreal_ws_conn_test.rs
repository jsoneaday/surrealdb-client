use surrealdb_client::connection::surreal_ws_conn::SurrealWsConnection;

const HOST: &str = "localhost";
const PORT: usize = 8000;

#[tokio::test]
async fn int_verify_connect_makes_connection_to_surrealdb() {       
    let mut conn: SurrealWsConnection = SurrealWsConnection::new(HOST, PORT, false); 
    let result = conn.connect().await;
    
    assert!(result.is_ok());
    conn.disconnect().await;
}

#[tokio::test]
async fn int_verify_exec_sends_message_without_error() {
    todo!()
}

#[tokio::test]
async fn int_verify_disconnect_is_disconnecting() {
    todo!("needs test, but after some of the actual queries are built out");
}