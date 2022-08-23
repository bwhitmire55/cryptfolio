use sqlite3::Connection;
use crate::database::entry::DatabaseEntry;
use crate::error::CryptfolioError;

#[derive(Default)]
pub struct CoinAccount {
    id: String,
    coin: String,
    platform: String, 
}

impl CoinAccount {
    pub fn new(id: String, coin: String, platform: String) -> CoinAccount {
        CoinAccount { 
            id: id, coin: coin, platform: platform 
        }
    }
}

impl DatabaseEntry for CoinAccount {
    fn write(&self, dbh: &Connection) -> Result<(), CryptfolioError> {
        let mut statement = dbh.prepare(
            "INSERT INTO accounts (id, coin, platform) VALUES(?, ?, ?)"
        ).unwrap();
        statement.bind(1, self.id.as_str()).unwrap();
        statement.bind(2, self.coin.as_str()).unwrap();
        statement.bind(3, self.platform.as_str()).unwrap();
        statement.next().unwrap();
        Ok(())
    }
}