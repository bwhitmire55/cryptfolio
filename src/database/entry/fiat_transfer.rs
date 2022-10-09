/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// fiat_transfer.rs
/// 
/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// Description:
///     Database entry for a fiat transfer from one account to another.
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
pub struct FiatTransfer {
    id: String,
    date: String,
    origin: String,
    destination: String,
    amount: f64,
}

impl FiatTransfer {
    pub fn new(id: String, date: String, origin: String, destination: String, amount: f64) -> FiatTransfer {
        FiatTransfer { 
            id: id, date: date, origin: origin, destination: destination, amount: amount 
        }
    }
}

impl DatabaseEntry for FiatTransfer {
    fn write(&self, dbh: &Connection) -> Result<(), CryptfolioError> {
        let mut statement = dbh.prepare(
            "INSERT INTO fiat_transfers (id, date, origin, destination, amount) VALUES(?, ?, ?, ?, ?)"
        ).unwrap();
        statement.bind(1, self.id.as_str()).unwrap();
        statement.bind(2, self.date.as_str()).unwrap();
        statement.bind(3, self.origin.as_str()).unwrap();
        statement.bind(4, self.destination.as_str()).unwrap();
        statement.bind(5, self.amount).unwrap();
        statement.next().unwrap();
        Ok(())
    }
}