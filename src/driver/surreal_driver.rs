use crate::connection::surreal_ws_conn::SurrealWsConnection;
use crate::connection::model::method::Method;
use std::collections::BTreeMap;
use surrealdb::sql::{Object, Value};
use crate::connection::model::rpcrequest::{RpcParams};

type TungsteniteResult = Result<tungstenite::Message, tungstenite::Error>;

#[allow(unused)]
pub struct SurrealDriver {
    conn: SurrealWsConnection
}

#[allow(unused)]
impl SurrealDriver {
    fn new(conn: SurrealWsConnection) -> Self {
        Self {
            conn
        }
    }

    async fn ping(&mut self) -> TungsteniteResult{        
        self.conn.rpc(Method::Ping, RpcParams::Objects(vec![Object(BTreeMap::new())])).await
    }

    async fn info(&mut self) -> TungsteniteResult {
        self.conn.rpc(Method::Info, RpcParams::Array(Vec::new())).await
    }

    async fn sign_in(&mut self, username: &str, password: &str) -> TungsteniteResult {
        let mut sign_in: BTreeMap<String, Value> = BTreeMap::new();
        sign_in.insert("user".to_string(), username.into());
        sign_in.insert("pass".to_string(), password.into());

        self.conn.rpc(Method::SignIn, RpcParams::Objects(vec![Object::from(sign_in)])).await
    }

    async fn use_ns_db(&mut self, ns: &str, db: &str) -> TungsteniteResult {
        let mut ns_db_vals: BTreeMap<String, Value> = BTreeMap::new();
        ns_db_vals.insert("NS".to_string(), ns.into());
        ns_db_vals.insert("DB".to_string(), db.into());

        //self.conn.rpc(Method::Use, vec![Object(ns_db_vals)]).await
        self.conn.rpc(Method::Use, RpcParams::Array(vec![ns.to_string(), db.to_string()])).await
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    const HOST: &str = "localhost";
    const PORT: usize = 8000;

    async fn get_driver() -> SurrealDriver {
        let mut surreal_conn = SurrealWsConnection::new(&HOST, PORT, false);
        let _ = surreal_conn.connect().await;
        SurrealDriver::new(surreal_conn)
    }

    #[tokio::test]
    async fn driver_ping_succeeds() {
        let mut driver = get_driver().await;

        let result = driver.ping().await;
        println!("{:#?}", result);
    }

    #[tokio::test]
    async fn driver_info_returns_surreal_info() {
        let mut driver = get_driver().await;

        // must set call use_ns_db first
        let _ = driver.use_ns_db("test", "test").await;
        println!("called use");
        println!("calling info");
        let result = driver.info().await;

        println!("{:#?}", result);
    }

    #[tokio::test]
    async fn driver_sign_in_succeeds() {
        let mut driver = get_driver().await;

        let result = driver.sign_in("superduper", "superpass").await;

        assert_eq!(result.as_ref().is_ok(), true);
        println!("{:#?}", result.unwrap());
    }

    #[tokio::test]
    async fn driver_use_ns_db_succeeds() {
        let mut driver = get_driver().await;

        let result = driver.use_ns_db("test", "test").await;

        println!("{:#?}", result);
    }
}