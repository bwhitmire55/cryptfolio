use crate::{error::CryptfolioError, database::entry::DatabaseEntry};
use async_trait::async_trait;

#[async_trait]
pub trait SyncClient {
    async fn sync(&self) -> Result<Vec<Box<dyn DatabaseEntry + Send>>, CryptfolioError>;
}