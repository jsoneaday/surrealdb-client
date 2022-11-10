mod common;

use common::FIXTURES;

#[tokio::test]
async fn verify_connect_makes_connection_to_surrealdb() {       
    let mut fixture = FIXTURES.lock().unwrap();
    let fixture_items = fixture.instance.as_mut().unwrap();
    let result = fixture_items.conn.connect().await;
    
    assert!(result.is_ok());
    fixture_items.conn.disconnect().await;
}

#[tokio::test]
async fn verify_exec_sends_message_without_error() {
    todo!()
}

#[tokio::test]
async fn verify_disconnect_is_disconnecting() {
    todo!("needs test, but after some of the actual queries are built out");
}