pub mod entry;
pub mod script;

use sqlite3::Connection;
use crate::error::CryptfolioError;

pub struct Database {
    dbh: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Database, CryptfolioError> {
        if let Ok(dbh) = sqlite3::open(path) {
            match Database::create_default_tables(&dbh) {
                Ok(_) => {
                    Ok(Database {
                        dbh: dbh
                    })
                },
                Err(_) => { return Err(CryptfolioError::DatabaseQueryFailed("Failed to create default tables".to_string())); }
            }
        } else {
            Err(CryptfolioError::DatabaseConnectionFailed(path.to_string()))
        }
    }

    pub fn get_dbh(&self) -> &Connection {
        &self.dbh
    }

    fn create_default_tables(dbh: &Connection) -> Result<(), CryptfolioError> {
        match dbh.execute("
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
}