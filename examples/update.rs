// This example makes a connection to surrealdb and creates an instance of driver in preparation of making calls
// use run-docker.sh script on root of library to startup a docker image of surreal

use std::collections::BTreeMap;
use tungstenite::Message;
use serde::Deserialize;
use surrealdb_client::connection::model::rpcresponse::RpcResponse;
use surrealdb_client::connection::surreal_ws_conn::SurrealWsConnection;
use surrealdb_client::driver::surreal_driver::SurrealDriver;

pub const HOST: &str = "localhost";
pub const PORT: usize = 8000;
pub const USER_NAME: &str = "superduper";
pub const PASSWORD: &str = "superpass";
pub const NS: &str = "test";
pub const DB: &str = "test";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Employee {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub company: Option<String>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Company {
    pub id: String,
    pub name: String
}

#[tokio::main]
async fn main() {
    let mut driver = set_up(NS, DB).await;

    // first create something to select 
    let _ = driver.query("
        create Company set name = 'Acme'; \
        create Employee set firstName = 'Dave', lastName = 'Finny';
    ", BTreeMap::new()).await;
      
    // now select it
    let mut args = BTreeMap::new();
    args.insert("first_name".to_string(), "Dave".to_string());    
    let result = driver.query("update Employee set firstName = 'David' where firstName = $first_name", args).await;
    
    let message: Message = result.unwrap();
    let result_inst: Result<RpcResponse<Employee>, serde_json::Error> = match message {
        Message::Text(txt) => {
            serde_json::from_str(txt.as_str())
        },
        _ => {
            serde_json::from_str("")
        }
    };
    
    let rpc_result = result_inst.unwrap();
    println!("rpc_result {:#?}", rpc_result);

    clean_up(&mut driver, NS).await;
    driver.disconnect().await;
}

async fn set_up(ns: &str, db: &str) -> SurrealDriver { 
    let mut surreal_conn = SurrealWsConnection::new(&HOST, PORT, false);
    let _ = surreal_conn.connect().await;
    
    let mut driver = SurrealDriver::new(surreal_conn);

    let _ = driver.sign_in(USER_NAME, PASSWORD).await;
    let _ = driver.use_ns_db(ns, db).await;

    clean_up(&mut driver, ns).await;

    driver
}

async fn clean_up(driver: &mut SurrealDriver, ns: &str) {
    let args = BTreeMap::new();
    let _ = driver.query(format!("remove namespace {}", ns).as_str(), args).await;
}