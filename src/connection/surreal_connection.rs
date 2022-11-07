use async_trait::async_trait;
use super::error::SurrealError;

#[async_trait]
pub trait SurrealConnection {
    async fn connect(&mut self) -> Result<(), SurrealError>;
    //fn disconnect();
    //fn exec();
}