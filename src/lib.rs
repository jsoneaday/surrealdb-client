pub mod connection {
    pub mod error;
    pub mod surreal_ws_conn;
    pub mod model {
        pub mod rpcrequest;
        pub mod rpcresponse;
        pub mod method;
        pub mod surrealresult;
    }
}
pub mod driver {
    pub mod surreal_driver;
}


#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        
    }
}
