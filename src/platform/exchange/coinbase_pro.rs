/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// coinbase_pro.rs
/// 
/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// Description:
///     Coinbase Pro (Exchange) implementation for syncing transaction history.
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

use crate::database::entry::{CoinAccount, CoinOrder, CoinTransfer};
use crate::database::entry::DatabaseEntry;
use crate::error::CryptfolioError;
use crate::platform::SyncClient;
use async_trait::async_trait;
use coinbase::pro::account::Account;
use coinbase::pro::client::Client;
use coinbase::pro::order::Order;
use coinbase::pro::transfer::Transfer;

pub struct CoinbasePro {
    api_key: String,
    api_secret: String,
    passphrase: String,
    client: Client,
}

impl CoinbasePro {
    pub fn new(api_key: &str, api_secret: &str, passphrase: &str) -> Result<CoinbasePro, CryptfolioError> {
        // const API_VERSION: &str = "2022-07-14";
        Ok(
            CoinbasePro {
                api_key: api_key.to_string(),
                api_secret: api_secret.to_string(),
                passphrase: passphrase.to_string(),
                client: Client::new(api_key, api_secret, passphrase),
            }
        )
    }

    pub fn get_api_key(&self) -> String {
        self.api_key.to_string()
    }

    pub fn get_api_secret(&self) -> String {
        self.api_secret.to_string()
    }

    pub fn get_api_passphrase(&self) -> String {
        self.passphrase.to_string()
    }

    fn process_fill(&self, fill: Order) -> Box<dyn DatabaseEntry + Send> {
        Box::new(CoinOrder::new(
            fill.order_id,
            fill.created_at,
            fill.product_id,
            fill.price.parse::<f64>().unwrap(),
            fill.size.parse::<f64>().unwrap(),
            fill.fee.parse::<f64>().unwrap(),
            fill.side,
            "Coinbase Pro".to_string()
        ))
    }

    fn process_transfer(&self, transfer: Transfer, account: &Account) -> Box<dyn DatabaseEntry + Send> {
        let origin: String;
        let destination: String;
        let fee: f64;

        match transfer.type_transfer.as_str() {
            "deposit" => {
                if let Some(_addr) = &transfer.details.as_ref().unwrap().crypto_address {
                    // deposit from external address
                    origin = String::new();
                    destination = account.id.to_string();
                    fee = f64::NAN;
                } else {
                    // deposit from coinbase
                    origin = transfer.details.as_ref().unwrap().coinbase_account_id.as_ref().unwrap().to_string();
                    destination = account.id.to_string();
                    fee = 0.0;
                }
            },
            "withdraw" => {
                if let Some(addr) = &transfer.details.as_ref().unwrap().sent_to_address {
                    // withdrawal to external address
                    origin = account.id.to_string();
                    destination = addr.to_string();
                    fee = transfer.details.as_ref().unwrap().fee.as_ref().unwrap().parse::<f64>().unwrap();
                } else {
                    // withdrawal to coinbase
                    origin = account.id.to_string();
                    destination = transfer.details.as_ref().unwrap().coinbase_account_id.as_ref().unwrap().to_string();
                    fee = 0.0;
                }
            },
            _ => { origin = "".to_string(); destination = "".to_string(); fee = f64::NAN; }
        }

        Box::new(CoinTransfer::new(
            transfer.id,
            transfer.created_at,
            origin,
            destination,
            account.currency.to_string(),
            transfer.amount.parse::<f64>().unwrap(),
            fee
        ))
    }
}

#[async_trait]
impl SyncClient for CoinbasePro {
    async fn sync(&self) -> Result<Vec<Box<dyn DatabaseEntry + Send>>, CryptfolioError> {
        let mut result = Vec::<Box<dyn DatabaseEntry + Send>>::new();
        match self.client.fetch_accounts().await {
            Ok(accounts) => {
                for account in accounts {
                    result.push(Box::new(CoinAccount::new(
                        account.id.to_string(),
                        account.currency.to_string(),
                        "Coinbase Pro".to_string()
                    )));
                    match self.client.fetch_filled_orders_pag(&format!("{}-USD", account.currency)).await {
                        Ok(fills) => {
                            for fill in fills {
                                result.push(self.process_fill(fill));
                            }
                        },
                        Err(e) => {
                            return Err(CryptfolioError::CoinbaseProAPIError(e.to_string()));
                        }
                    }
                    match self.client.fetch_transfers(&account.id).await {
                        Ok(transfers) => {
                            for transfer in transfers {
                                result.push(self.process_transfer(transfer, &account));
                            }
                        },
                        Err(e) => {
                            return Err(CryptfolioError::CoinbaseProAPIError(e.to_string()));
                        }
                    }
                }
            },
            Err(e) => {
                return Err(CryptfolioError::CoinbaseProAPIError(e.to_string()));
            }
        }
        
        Ok(result)
    }
}