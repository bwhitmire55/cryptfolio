pub mod entry;
pub mod script;

use sqlite3::Connection;
use crate::error::CryptfolioError;
use crate::database::script::DatabaseScript;

pub struct Database {
    dbh: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Database, CryptfolioError> {
        if let Ok(dbh) = sqlite3::open(path) {
            match DatabaseScript::create_default_tables(&dbh) {
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
}