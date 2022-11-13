pub mod connection {
    pub mod error;
    pub mod surreal_ws_conn;
    pub mod model {
        pub mod rpcrequest;
    }
}
pub mod driver {
    pub mod surreal_driver;
}
pub mod sql;


#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        
    }
}
