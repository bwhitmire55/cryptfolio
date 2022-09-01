/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// coin_transfer.rs
/// 
/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// Description:
///     Database entry for a coin transfer from one account to another.
/// 
/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// Usage:
/// 
/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// Notes:
/// 
/// ///////////////////////////////////////////////////////////////////////////////////////////////

use sqlite3::Connection;
use crate::database::entry::DatabaseEntry;
use crate::error::CryptfolioError;

#[derive(Default)]
pub struct CoinTransfer {
    id: String,
    date: String,
    origin: String,
    destination: String,
    coin: String,
    unit_size: f64,
    fee: f64,
}

impl CoinTransfer {
    pub fn new(id: String, date: String, origin: String, destination: String, coin: String, unit_size: f64, fee: f64) -> CoinTransfer {
        CoinTransfer { 
            id: id, date: date, origin: origin, destination: destination, coin: coin, unit_size: unit_size, fee: fee 
        }
    }
}

impl DatabaseEntry for CoinTransfer {
    fn write(&self, dbh: &Connection) -> Result<(), CryptfolioError> {
        let mut statement = dbh.prepare(
            "INSERT INTO transfers (id, date, origin, destination, coin, unit_size, fee) VALUES (?, ?, ?, ?, ?, ?, ?)"
        ).unwrap();
        statement.bind(1, self.id.as_str()).unwrap();
        statement.bind(2, self.date.as_str()).unwrap();
        statement.bind(3, self.origin.as_str()).unwrap();
        statement.bind(4, self.destination.as_str()).unwrap();
        statement.bind(5, self.coin.as_str()).unwrap();
        statement.bind(6, self.unit_size).unwrap();
        statement.bind(7, self.fee).unwrap();
        statement.next().unwrap();
        Ok(())
    }
}