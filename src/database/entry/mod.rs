pub mod coin_account;
pub mod coin_order;
pub mod coin_reward;
pub mod coin_transfer;
pub mod fiat_transfer;
pub use coin_account::CoinAccount;
pub use coin_order::CoinOrder;
pub use coin_reward::CoinReward;
pub use coin_transfer::CoinTransfer;
pub use fiat_transfer::FiatTransfer;

use sqlite3::Connection;
use crate::error::CryptfolioError;

pub trait DatabaseEntry {
    fn write(&self, dbh: &Connection) -> Result<(), CryptfolioError>;
}

// Workaround for returning something other than a Result<Box<dyn DatabaseEntry + Send>> from SyncClient::sync()
// in order to support trades within the Coinbase client. Should be looked into...

pub struct Dud {}
impl DatabaseEntry for Dud {
    fn write(&self, _dbh: &Connection) -> Result<(), CryptfolioError> {
        Ok(())
    }
}