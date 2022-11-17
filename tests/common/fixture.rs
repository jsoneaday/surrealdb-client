// This file is work in progress, do not use


use surrealdb_client::{connection::surreal_ws_conn::SurrealWsConnection, driver::surreal_driver::SurrealDriver};
use std::{sync::Mutex, collections::BTreeMap};
use once_cell::sync::Lazy;

// obviously SurrealDB must be running here
pub const HOST: &str = "localhost";
pub const PORT: usize = 8000;
pub const USER_NAME: &str = "superduper";
pub const PASSWORD: &str = "superpass";
pub const NS: &str = "test";
pub const DB: &str = "test";

#[allow(unused)]
pub struct Fixture {
    pub instance: Option<FixtureItem>
}

#[allow(unused)]
impl Fixture {
    fn get_singleton(mut self) -> Self {
        if let None = self.instance {
            self.instance = Some(FixtureItem {
                surreal_conn: SurrealWsConnection::new(HOST, PORT, false)
            });
        }
        self
    }

    pub async fn clean_db(&mut self) {
        println!("start clean_db");
      
        let mut conn: SurrealWsConnection = SurrealWsConnection::new(HOST, PORT, false);
        let _ = conn.connect().await;
        let mut driver = SurrealDriver::new(conn);

        let _ = driver.sign_in(USER_NAME, PASSWORD).await;
        let _ = driver.use_ns_db(NS, DB).await;
        let _ = driver.query("
            remove namespace 'test'; \
        ", BTreeMap::new());

        println!("end clean_db");
    }
}

pub struct FixtureItem {
    pub surreal_conn: SurrealWsConnection
}

#[allow(unused)]
pub static FIXTURES: Lazy<Mutex<Fixture>> = Lazy::new(|| {
    let mut fixture = Fixture {
        instance: None
    };
    fixture = fixture.get_singleton();

    Mutex::new(fixture)
});