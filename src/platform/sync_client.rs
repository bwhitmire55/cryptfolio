use crate::{error::CryptfolioError, database::entry::{DatabaseEntry, PlatformConnection}};
use async_trait::async_trait;

#[async_trait]
pub trait SyncClient {
    fn get_name(&self) -> &str;
    fn get_connection(&self, nickname: &String) -> PlatformConnection;
    async fn sync(&self) -> Result<Vec<Box<dyn DatabaseEntry + Send>>, CryptfolioError>;
}