mod common {
    pub mod fixture;
    pub mod datamodel;
}

use tungstenite::Message;
use std::collections::BTreeMap;
use surrealdb_client::connection::surreal_ws_conn::SurrealWsConnection;
use surrealdb_client::driver::surreal_driver::SurrealDriver;
use surrealdb_client::connection::model::rpcresponse::RpcResponse;
use common::fixture::{HOST, PORT};
use common::datamodel::{employee::Employee, company::Company};

async fn get_driver() -> SurrealDriver {
    let mut surreal_conn = SurrealWsConnection::new(&HOST, PORT, false);
    let _ = surreal_conn.connect().await;
    SurrealDriver::new(surreal_conn)
}

#[tokio::test]
async fn driver_query_create_single_employee_succeeds() {
    let mut driver = get_driver().await;

    let _ = driver.sign_in("superduper", "superpass").await;
    let _= driver.use_ns_db("test", "test").await;
    let result = driver.query("CREATE Employee SET firstName = 'John', lastName = 'Thompson'", BTreeMap::new()).await;

    let message: Message = result.unwrap();
    let result_obj: Result<RpcResponse<Employee>, serde_json::Error> = match message {
    //let json = match message {
        Message::Text(txt) => {
            println!("{}", txt.as_str());
            serde_json::from_str(txt.as_str())
            //txt
        },
        _ => {
            serde_json::from_str("")
            //"".to_string()
        }
    };
    //println!("test: {}", json);
    println!("test: {:#?}", result_obj);
}

#[tokio::test]
async fn driver_query_create_company_employee_select_back_succeeds() {
    let mut driver = get_driver().await;

    let _ = driver.sign_in("superduper", "superpass").await;
    let _= driver.use_ns_db("test", "test").await;
    let _ = driver.query("CREATE Company SET name = 'Super Big Corporation'", BTreeMap::new()).await;

    //println!("{:#?}", result);
}