mod common;

use std::{collections::BTreeMap};

use surrealdb_client::connection::surreal_ws_conn::Method;
use surrealdb::sql::Object;
use common::FIXTURES;

#[tokio::test]
async fn rpc_ping_completes_successfully() {
    let mut fixture = FIXTURES.lock().unwrap();
    let fixture_items = fixture.instance.as_mut().unwrap();
    let _ = fixture_items.conn.connect().await;

    let result = fixture_items.conn.rpc(Method::Ping, vec![Object(BTreeMap::new())]).await;

    println!("{:#?}", result.as_ref().unwrap());
    assert!(result.is_ok());
}
