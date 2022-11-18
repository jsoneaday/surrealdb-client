// This example creates a new record using the 'query' function
// note it is possible to make any surrealdb call using this function
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
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub age: u64
}

#[tokio::main]
async fn main() {
    let mut conn = SurrealWsConnection::new(HOST, PORT, false);
    let _ = conn.connect().await;
    let mut driver = SurrealDriver::new(conn);

    // you must sign in if you are using username and password
    let _ = driver.sign_in("superduper", "superpass").await;
    // you must always indicate a namespace and database to use
    let _= driver.use_ns_db("test", "test").await;

    // any custom query can be run when using the query function
    let result = driver.query("
        create Person \
        set firstName = 'John', lastName = 'Thompson', age = 18
    ", BTreeMap::new()).await;

    let message: Message = result.unwrap();
    // parse the json into a Rust type
    let result_inst: Result<RpcResponse<Person>, serde_json::Error> = match message {
        Message::Text(txt) => {
            println!("{}", txt.as_str());
            serde_json::from_str(txt.as_str())
        },
        _ => {
            serde_json::from_str("")
        }
    };
    
    let rpc_result = result_inst.unwrap();
    println!("{:#?}", rpc_result);

    driver.disconnect().await;
}
