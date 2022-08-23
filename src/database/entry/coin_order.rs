use sqlite3::Connection;
use crate::database::entry::DatabaseEntry;
use crate::error::CryptfolioError;

#[derive(Default)]
pub struct CoinOrder {
    id: String,
    date: String,
    pair: String,
    unit_price: f64,
    unit_size: f64,
    fee: f64,
    side: String,
    platform: String,
}

impl CoinOrder {
    pub fn new(id: String, date: String, pair: String, unit_price: f64, unit_size: f64, fee: f64, side: String, platform: String) -> CoinOrder {
        CoinOrder {
            id: id, date: date, pair: pair, unit_price: unit_price, unit_size: unit_size, fee: fee, side: side, platform: platform
        }
    }
}

impl DatabaseEntry for CoinOrder {
    fn write(&self, dbh: &Connection) -> Result<(), CryptfolioError> {
        let mut statement = dbh
            .prepare(
            "INSERT INTO orders (id, date, pair, unit_price, unit_size, fee, side, platform) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        ).unwrap();
        statement.bind(1, self.id.as_str()).unwrap();
        statement.bind(2, self.date.as_str()).unwrap();
        statement.bind(3, self.pair.as_str()).unwrap();
        statement.bind(4, self.unit_price).unwrap();
        statement.bind(5, self.unit_size).unwrap();
        statement.bind(6, self.fee).unwrap();
        statement.bind(7, self.side.as_str()).unwrap();
        statement.bind(8, self.platform.as_str()).unwrap();
        statement.next().unwrap();
        Ok(())
    }
}