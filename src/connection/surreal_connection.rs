use std::time::Duration;
use async_trait::async_trait;


#[async_trait]
pub trait SurrealConnection {
    async fn connect(&self, timeout: Duration);
    //fn disconnect();
    //fn exec();
}