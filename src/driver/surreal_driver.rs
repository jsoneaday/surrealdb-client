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
    pub fn new(conn: SurrealWsConnection) -> Self {
        Self {
            conn
        }
    }

    pub async fn disconnect(&mut self) {
        self.conn.disconnect().await;
    }

    pub async fn ping(&mut self) -> TungsteniteResult{        
        self.conn.rpc(Method::Ping, RpcParams::Objects(vec![Object(BTreeMap::new())])).await
    }

    pub async fn info(&mut self) -> TungsteniteResult {
        self.conn.rpc(Method::Info, RpcParams::Array(Vec::new())).await
    }

    pub async fn sign_in(&mut self, username: &str, password: &str) -> TungsteniteResult {
        let mut sign_in: BTreeMap<String, Value> = BTreeMap::new();
        sign_in.insert("user".to_string(), username.into());
        sign_in.insert("pass".to_string(), password.into());

        self.conn.rpc(Method::SignIn, RpcParams::Objects(vec![Object::from(sign_in)])).await
    }

    pub async fn use_ns_db(&mut self, ns: &str, db: &str) -> TungsteniteResult {
        let mut ns_db_vals: BTreeMap<String, Value> = BTreeMap::new();
        ns_db_vals.insert("NS".to_string(), ns.into());
        ns_db_vals.insert("DB".to_string(), db.into());

        //self.conn.rpc(Method::Use, vec![Object(ns_db_vals)]).await
        self.conn.rpc(Method::Use, RpcParams::Array(vec![ns.to_string(), db.to_string()])).await
    }

    pub async fn query(&mut self, query: &str, args: BTreeMap<String, String>) -> TungsteniteResult {
        self.conn.rpc(Method::Query, RpcParams::Query((query.to_string(), args))).await
    } 
}

