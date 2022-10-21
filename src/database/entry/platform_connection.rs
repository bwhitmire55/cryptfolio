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

use sqlite3::Connection;
use crate::database::entry::DatabaseEntry;
use crate::error::CryptfolioError;

#[derive(Default)]
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

#[derive(Default)]
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
}

impl DatabaseEntry for PlatformConnection {
    fn write(&self, dbh: &Connection) -> Result<(), CryptfolioError> {
        let mut statement = dbh.prepare(
            "INSERT INTO connections (nickname, platform) VALUES(?, ?)"
        ).unwrap();
        statement.bind(1, self.nickname.as_str()).unwrap();
        statement.bind(2, self.platform.as_str()).unwrap();
        statement.next().unwrap();

        statement = dbh.prepare(
            "SELECT id FROM connections WHERE nickname = ? AND platform = ?"
        ).unwrap();
        statement.bind(1, self.nickname.as_str()).unwrap();
        statement.bind(2, self.platform.as_str()).unwrap();
        statement.next().unwrap();
        let id = statement.read::<i64>(0).unwrap();

        for data in &self.connection_data {
            statement = dbh.prepare(
                "INSERT INTO connection_data (connection, key, value) VALUES(?, ?, ?)"
            ).unwrap();
            statement.bind(1, id).unwrap();
            statement.bind(2, data.key.as_str()).unwrap();
            statement.bind(3, data.value.as_str()).unwrap();
            statement.next().unwrap();
        }

        Ok(())
    }
}