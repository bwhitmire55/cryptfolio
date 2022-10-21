use crate::error::CryptfolioError;
use crate::recording::CoinRecord;

use sqlite3::Connection;
use sqlite3::State;

pub struct DatabaseScript {}

impl DatabaseScript {
    pub fn create_default_tables(dbh: &Connection) -> Result<(), CryptfolioError> {
        match dbh.execute("
            CREATE TABLE IF NOT EXISTS connections (
                id INTEGER PRIMARY KEY,
                nickname TEXT,
                platform TEXT,
                UNIQUE(nickname, platform)
            );

            CREATE TABLE IF NOT EXISTS connection_data (
                connection INTEGER,
                key STRING,
                value STRING
            );

            CREATE TABLE IF NOT EXISTS accounts (
                id TEXT UNIQUE,
                coin TEXT,
                platform TEXT   
            );

            CREATE TABLE IF NOT EXISTS fiat_transfers (
                id TEXT,
                date TEXT,
                origin TEXT,
                destination TEXT,
                amount REAL
            );

            CREATE TABLE IF NOT EXISTS orders (
                id TEXT,
                date TEXT,
                pair TEXT,
                unit_price REAL,
                unit_size REAL,
                fee REAL,
                side TEXT,
                platform TEXT
            );
            
            CREATE TABLE IF NOT EXISTS rewards (
                id TEXT,
                date TEXT,
                coin TEXT,
                unit_price REAL,
                unit_size REAL,
                type TEXT,
                description TEXT
            );
            
            CREATE TABLE IF NOT EXISTS transfers (
                id TEXT,
                date TEXT,
                origin TEXT,
                destination TEXT,
                coin TEXT,
                unit_size REAL,
                fee REAL
            );
        ")
        {
            Ok(_) => { Ok(()) },
            Err(_) => { return Err(CryptfolioError::DatabaseQueryFailed("Failed to create default tables".to_string())); }
        }
    }

    pub fn fetch_coin_record(dbh: &Connection, coin: String) -> CoinRecord {
        let mut record = CoinRecord::new();
        let mut statement = dbh.prepare("
            SELECT date, side, unit_price, unit_size, fee
            FROM orders 
            WHERE pair = ?
            UNION
            
            SELECT date,
            CASE WHEN description IS NOT NULL THEN 'buy' END side,
            unit_price, unit_size,
            '0.0' AS fee
            FROM rewards
            WHERE coin = ?
            UNION
            
            SELECT date,
            'sell' AS side,
            '0.0' AS unit_price,
            fee AS unit_size,
            '0.0' AS fee
            FROM transfers
            WHERE coin = ?
            AND fee > 0.0
            
            ORDER BY date ASC
        ").unwrap();
        statement.bind(1, format!("{}-USD", coin).as_str()).unwrap();
        statement.bind(2, coin.as_str()).unwrap();
        statement.bind(3, coin.as_str()).unwrap();
        while let State::Row = statement.next().unwrap() {
            match statement.read::<String>(1).unwrap().as_str() {
                "buy" => {
                    record.add_buy(
                        statement.read::<String>(0).unwrap(),
                        statement.read::<f64>(2).unwrap(), 
                        statement.read::<f64>(3).unwrap(),
                        statement.read::<f64>(4).unwrap()
                    );
                },
                "sell" => {
                    record.add_sell(
                        statement.read::<String>(0).unwrap(),
                        statement.read::<f64>(2).unwrap(),
                        statement.read::<f64>(3).unwrap(),
                        statement.read::<f64>(4).unwrap()
                    );
                },
                _ => {
                    
                }
            }
        }
        record.update();
        record
    }

    pub fn update_default_values(dbh: &Connection) {
        // Updating fiat_transfers table to replace 'Coinbase Pro' destination entries
        dbh.execute("
            UPDATE fiat_transfers
            SET destination = (
                SELECT id
                FROM accounts
                WHERE platform = 'Coinbase Pro'
                AND platform IS NOT NULL
                LIMIT 1
            )
            WHERE destination = 'Coinbase Pro'
        ").unwrap();

        // Updating transfers table to replace 'Coinbase Pro' origin & destination entries
        dbh.execute("
            UPDATE transfers
            SET origin = (
                SELECT id
                FROM accounts
                WHERE platform = 'Coinbase Pro'
                AND platform IS NOT NULL
                LIMIT 1
            )
            WHERE origin = 'Coinbase Pro'
        ").unwrap();

        dbh.execute("
            UPDATE transfers
            SET destination = (
                SELECT id
                FROM accounts
                WHERE platform = 'Coinbase Pro'
                AND platform IS NOT NULL
                LIMIT 1
            )
            WHERE destination = 'Coinbase Pro'
        ").unwrap();
    }
}