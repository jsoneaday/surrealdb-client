use surrealdb_client::connection::surreal_ws_conn::SurrealWsConnection;
use std::sync::Mutex;
use once_cell::sync::Lazy;

// obviously SurrealDB must be running here
pub const HOST: &str = "localhost";
pub const PORT: usize = 8000;

pub struct Fixture {
    pub instance: Option<FixtureItems>    
}

impl Fixture {
    pub fn singleton(mut self) -> Self {
        if let None = self.instance {
            self.instance = Some(FixtureItems {
                conn: SurrealWsConnection::new(HOST, PORT, false)
            });
        }
        self
    }
}

pub struct FixtureItems {
    pub conn: SurrealWsConnection
}

// may not use this for now
#[allow(unused)]
pub static FIXTURES: Lazy<Mutex<Fixture>> = Lazy::new(|| {
    let mut empty_fixture = Fixture {
        instance: None
    };
    let set_fixture = empty_fixture.singleton();

    Mutex::new(set_fixture)
});