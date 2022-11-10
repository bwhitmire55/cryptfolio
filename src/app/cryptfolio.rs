/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// cryptfolio.rs
/// 
/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// Description:
///     Main structure for portfolio management and user interaction.
/// 
/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// Usage:
///     Create an instance of a CryptfolioApp to manage aspects of a user's portfolio.
/// 
/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// Notes:
/// 
/// ///////////////////////////////////////////////////////////////////////////////////////////////

use crate::error::CryptfolioError;
use crate::database::{Database, script::DatabaseScript, entry::DatabaseEntry};
use crate::platform::SyncClient;
use crate::recording::CoinRecord;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct CryptfolioApp {
    database: Database,
    connected_platforms: RefCell<HashMap<String, Rc<Box<dyn SyncClient>>>>,
}

impl CryptfolioApp {
    pub fn new(db_path: &str) -> Result<CryptfolioApp, CryptfolioError> {
        // Init database
        let db: Database;
        match Database::new(db_path) {
            Ok(database) => { db = database }
            Err(_) => { return Err(CryptfolioError::DatabaseConnectionFailed(db_path.to_string())) }
        }

        // Load existing connections from database
        

        Ok(
            CryptfolioApp {
                database: db,
                connected_platforms: RefCell::new(HashMap::new()),
            }
        )
    }

    pub fn add_platform<T: SyncClient + 'static>(&self, nickname: &str, platform: T) -> Result<Rc<Box<dyn SyncClient>>, CryptfolioError> {
        let key = format!("{}:{}", platform.get_name(), nickname.to_string());
        if let Some(_) = self.connected_platforms.borrow_mut().insert(key.to_string(), Rc::new(Box::new(platform))) {
            return Err(CryptfolioError::PlatformAlreadyExists);
        } 

        let handle = self.connected_platforms
            .borrow()
            .get(&key)
            .map(|x| x.clone())
            .unwrap();

        handle.get_connection(&nickname.to_string()).write(self.database.get_dbh()).unwrap();
        Ok(handle)
    }

    pub async fn sync_platform(&self, platform: Rc<Box<dyn SyncClient>>) -> Result<(), CryptfolioError> {
        match platform.sync().await {
            Ok(result) => {
                for db_entry in result {
                    if let Err(e) = db_entry.write(self.database.get_dbh()) {
                        return Err(CryptfolioError::DatabaseWriteError(e.to_string()));
                    }
                }
                DatabaseScript::update_default_values(self.database.get_dbh());
                Ok(())
            },
            Err(e) => {
                return Err(CryptfolioError::CoinbaseAPIError(e.to_string()));
            }
        }        
    }

    pub fn add_transaction(&self, transaction: impl DatabaseEntry) -> Result<(), CryptfolioError> {
        if let Err(e) = transaction.write(self.database.get_dbh()) {
            return Err(CryptfolioError::DatabaseWriteError(e.to_string()));
        }
        Ok(())
    }

    pub fn get_coin_record(&self, coin: String) -> CoinRecord {
        DatabaseScript::fetch_coin_record(self.database.get_dbh(), coin)
    }

    pub fn get_connections(&self) -> Vec<Rc<Box<dyn SyncClient>>> {
        DatabaseScript::fetch_connections(self.database.get_dbh())
    }
}