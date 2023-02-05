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
pub mod router {
    pub mod message_router;
    pub mod message;
    pub mod msg_router_manager;
}
pub mod common_tests {
    pub mod datamodel;
    pub mod fixtures;
}


#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        
    }
}
