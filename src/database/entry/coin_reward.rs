use sqlite3::Connection;
use crate::database::entry::DatabaseEntry;
use crate::error::CryptfolioError;

#[derive(Default)]
pub struct CoinReward {
    id: String,
    date: String,
    coin: String,
    unit_price: f64,
    unit_size: f64,
    r#type: String,
    description: String,
}

impl CoinReward {
    pub fn new(id: String, date: String, coin: String, unit_price: f64, unit_size: f64, r#type: String, description: String) -> CoinReward {
        CoinReward {
             id: id, date: date, coin: coin, unit_price: unit_price, unit_size: unit_size, r#type: r#type, description: description 
        }
    }
}

impl DatabaseEntry for CoinReward {
    fn write(&self, dbh: &Connection) -> Result<(), CryptfolioError> {
        let mut statement = dbh.prepare(
            "INSERT INTO rewards (id, date, coin, unit_price, unit_size, type, description) VALUES (?, ?, ?, ?, ?, ?, ?)"
        ).unwrap();
        statement.bind(1, self.id.as_str()).unwrap();
        statement.bind(2, self.date.as_str()).unwrap();
        statement.bind(3, self.coin.as_str()).unwrap();
        statement.bind(4, self.unit_price).unwrap();
        statement.bind(5, self.unit_size).unwrap();
        statement.bind(6, self.r#type.as_str()).unwrap();
        statement.bind(7, self.description.as_str()).unwrap();
        statement.next().unwrap();
        Ok(())
    }
}