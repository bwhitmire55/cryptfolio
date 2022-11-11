/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// platform_connection.rs
/// 
/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// Description:
///     Database entry for a platform's connection.
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

use serde::{Serialize, Deserialize};
use sqlite3::Connection;
use std::rc::Rc;
use crate::database::entry::DatabaseEntry;
use crate::error::CryptfolioError;
use crate::platform::SyncClient;
use crate::platform::blockchain::Solana;
use crate::platform::exchange::Coinbase;
use crate::platform::exchange::CoinbasePro;

#[derive(Serialize, Deserialize)]
pub struct PlatformConnectionData {
    pub key: String,
    pub value: String,
}

impl PlatformConnectionData {
    pub fn new(key: String, value: String) -> PlatformConnectionData {
        PlatformConnectionData {
            key: key, value: value
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlatformConnection {
    pub nickname: String,
    pub platform: String, 
    pub connection_data: Vec<PlatformConnectionData>,
}

impl PlatformConnection {
    pub fn new(nickname: String, platform: String, connection_data: Vec<PlatformConnectionData>) -> PlatformConnection {
        PlatformConnection { 
            nickname: nickname, platform: platform, connection_data: connection_data
        }
    }

    pub fn to_concrete_type(&self) -> Rc<Box<dyn SyncClient>> {
        match self.platform.as_str() {
            "Coinbase" => {
                return Rc::new(Box::new(Coinbase::new(
                    &self.connection_data[0].value,
                    &self.connection_data[1].value
                ).unwrap()));
            },
            "Coinbase Pro" => {
                return Rc::new(Box::new(CoinbasePro::new(
                    &self.connection_data[0].value,
                    &self.connection_data[1].value,
                    &self.connection_data[2].value
                ).unwrap()));
            },
            "Solana" => {
                return Rc::new(Box::new(Solana::new(
                    self.connection_data[0].value.to_string()
                ).unwrap()));
            },
            _ => {
                panic!("Could not convert PlatformConnection to concrete type.");
            }
        }
    }
}

impl DatabaseEntry for PlatformConnection {
    fn write(&self, dbh: &Connection) -> Result<(), CryptfolioError> {
        let mut statement = dbh.prepare(
            "INSERT INTO connections (nickname, platform, object) VALUES(?, ?, ?)"
        ).unwrap();
        statement.bind(1, self.nickname.as_str()).unwrap();
        statement.bind(2, self.platform.as_str()).unwrap();
        statement.bind(3, &bincode::serialize(&self).unwrap() as &[u8]).unwrap();
        statement.next().ok();
        Ok(())
    }
}
