// This example makes a connection to surrealdb and creates an instance of driver in preparation of making calls
// it then signs into surrealdb with username and password (you must sign in before calling 'use' if you have a password set)
// use run-docker.sh script on root of library to startup a docker image of surreal

use surrealdb_client::connection::surreal_ws_conn::SurrealWsConnection;
use surrealdb_client::driver::surreal_driver::SurrealDriver;

pub const HOST: &str = "localhost";
pub const PORT: usize = 8000;
pub const USER_NAME: &str = "superduper";
pub const PASSWORD: &str = "superpass";
pub const NS: &str = "test";
pub const DB: &str = "test";

#[tokio::main]
async fn main() {
    let mut conn = SurrealWsConnection::new(HOST.to_string(), PORT, false);
    let _ = conn.connect().await;
    let mut driver = SurrealDriver::new(conn);

    // you must sign in if you are using username and password
    let sign_result = driver.sign_in(USER_NAME, PASSWORD).await;
    println!("{:#?}", sign_result);

    driver.disconnect().await;
}
