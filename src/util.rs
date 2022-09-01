/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// util.rs
/// 
/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// Description:
///     Utility functions and macros.
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

use crate::error::CryptfolioError;
use chrono::NaiveDateTime;

pub struct Util {}

impl Util {
    pub fn parse_date(date: &str) -> Result<NaiveDateTime, CryptfolioError> {
        match NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%SZ") {
            Ok(dt) => {
                return Ok(dt);
            },
            Err(_e) => {
                match NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%S%.6fZ") {
                    Ok(dt) => {
                        return Ok(dt);
                    },
                    Err(_e) => {
                        match NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S%.6f+00") {
                            Ok(dt) => {
                                return Ok(dt);
                            },
                            Err(e) => {
                                return Err(CryptfolioError::DateTimeParseError(e.to_string()));
                            }
                        }
                    }
                }
            }
        }
    }
}