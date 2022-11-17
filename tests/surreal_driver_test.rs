mod common {
    pub mod fixture;
    pub mod datamodel;
}

use tungstenite::Message;
use std::collections::BTreeMap;
use surrealdb_client::connection::surreal_ws_conn::SurrealWsConnection;
use surrealdb_client::driver::surreal_driver::SurrealDriver;
use surrealdb_client::connection::model::rpcresponse::RpcResponse;
use common::fixture::{HOST, PORT, USER_NAME, PASSWORD, NS, DB};
use common::datamodel::{employee::Employee};

async fn set_up() -> SurrealDriver { 
    let mut surreal_conn = SurrealWsConnection::new(&HOST, PORT, false);
    let _ = surreal_conn.connect().await;
    
    let mut driver = SurrealDriver::new(surreal_conn);

    let _ = driver.sign_in(USER_NAME, PASSWORD).await;
    let _ = driver.use_ns_db(NS, DB).await;
    let remove_result = driver.query("remove namespace 'test'", BTreeMap::new()).await;
    println!("remove_result {:#?}", remove_result);

    driver
}

async fn clean_up(driver: &mut SurrealDriver) {
    let _ = driver.query("
            remove namespace 'test'; \
        ", BTreeMap::new()).await;

    driver.disconnect().await;
}

#[tokio::test]
async fn driver_ping_succeeds() {
    let mut driver = set_up().await;

    let result = driver.ping().await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn driver_info_returns_surreal_info() {
    let mut driver = set_up().await;

    let _ = driver.use_ns_db("test", "test").await;    
    let result = driver.info().await;
    // todo: returns "{\"id\":\"54ece72a-f853-4d1d-bb64-22ca2bcda9ab\",\"result\":null}",
    // which seems wrong. Need to fix.
    println!("{:#?}", result);
}

#[tokio::test]
async fn driver_sign_in_succeeds() {
    let mut driver = set_up().await;

    let result = driver.sign_in("superduper", "superpass").await;

    assert_eq!(result.as_ref().is_ok(), true);
}

#[tokio::test]
async fn driver_use_ns_db_succeeds() {
    let mut driver = set_up().await;

    let result = driver.use_ns_db("test", "test").await;

    assert!(result.is_ok())
}

#[tokio::test]
async fn driver_query_create_single_employee_succeeds() {
    let mut driver = set_up().await;

    let _ = driver.sign_in("superduper", "superpass").await;
    let _= driver.use_ns_db("test", "test").await;
    let result = driver.query("
        create Employee \
        set firstName = 'John', lastName = 'Thompson'
    ", BTreeMap::new()).await;

    let message: Message = result.unwrap();
    let result_inst: Result<RpcResponse<Employee>, serde_json::Error> = match message {
        Message::Text(txt) => {
            println!("{}", txt.as_str());
            serde_json::from_str(txt.as_str())
        },
        _ => {
            serde_json::from_str("")
        }
    };
    
    let rpc_result = result_inst.unwrap();
    
    assert_eq!(rpc_result.result[0].status, "OK");
    assert_eq!(rpc_result.result.len(), 1 as usize);

    clean_up(&mut driver).await;
}

#[tokio::test]
#[ignore = "not read yet"]
async fn driver_query_create_company_employee_select_back_succeeds() {
    let mut driver = set_up().await;

    let _ = driver.sign_in("superduper", "superpass").await;
    let _= driver.use_ns_db("test", "test").await;
    let _ = driver.query("
        CREATE Company SET name = 'Super Big Corporation'; \
        CREATE Company SET name = 'Acme'; \
        CREATE Employee SET firstName = 'John' lastName = 'Brown'; \
    ", BTreeMap::new()).await;

    //println!("{:#?}", result);
}