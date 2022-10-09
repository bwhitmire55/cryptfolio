/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// coinbase.rs
/// 
/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// Description:
///     Coinbase (Exchange) implementation for syncing transaction history.
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

use std::collections::HashMap;

use crate::database::entry::{CoinAccount, CoinOrder, CoinTransfer, CoinReward, DatabaseEntry, Dud, FiatTransfer};
use crate::error::CryptfolioError;
use crate::platform::SyncClient;
use async_trait::async_trait;
use coinbase::base::client::Client;
use coinbase::base::transaction::Transaction;

pub struct Coinbase {
    api_key: String,
    api_secret: String,
    client: Client,
}

impl Coinbase {
    pub fn new(api_key: &str, api_secret: &str) -> Result<Coinbase, CryptfolioError> {
        const API_VERSION: &str = "2022-07-14";
        Ok(
            Coinbase {
                api_key: api_key.to_string(),
                api_secret: api_secret.to_string(),
                client: Client::new(api_key, api_secret, API_VERSION),
            }
        )
    }

    pub fn get_api_key(&self) -> String {
        self.api_key.to_string()
    }

    pub fn get_api_secret(&self) -> String {
        self.api_secret.to_string()
    }

    async fn process_transaction(&self, account_id: &String, trade_resources: &mut HashMap::<String, TradeResource>, transaction: Transaction) -> Result<Box<dyn DatabaseEntry + Send>, CryptfolioError> {
        match transaction.type_transaction.as_str() {
            "send" => {
                if let Some(from) = transaction.from {
                    if let Some(subtitle) = transaction.details.subtitle {
                        match subtitle.as_str() {
                            "From Coinbase Earn" => {
                                Ok(Box::new(CoinReward::new(
                                    transaction.id,
                                    transaction.created_at,
                                    transaction.amount.currency,
                                    transaction.native_amount.amount.parse::<f64>().unwrap() / transaction.amount.amount.parse::<f64>().unwrap(),
                                    transaction.amount.amount.parse::<f64>().unwrap(),
                                    "Learn & Earn".to_string(),
                                    "Coinbase Earn".to_string()
                                )))
                            },
                            "From Coinbase Rewards" => {
                                Ok(Box::new(CoinReward::new(
                                    transaction.id,
                                    transaction.created_at,
                                    transaction.amount.currency,
                                    transaction.native_amount.amount.parse::<f64>().unwrap() / transaction.amount.amount.parse::<f64>().unwrap(),
                                    transaction.amount.amount.parse::<f64>().unwrap(),
                                    "Staking".to_string(),
                                    "Coinbase Rewards".to_string()
                                )))
                            },
                            _ => {
                                Ok(Box::new(CoinTransfer::new(
                                    transaction.id,
                                    transaction.created_at,
                                    from.id.unwrap(),
                                    account_id.to_string(),
                                    from.currency.unwrap(),
                                    transaction.amount.amount.parse::<f64>().unwrap(),
                                    0.0
                                )))
                            },
                        }
                    } else {
                        Ok(Box::new(CoinTransfer::new(
                            transaction.id,
                            transaction.created_at,
                            from.id.unwrap(),
                            account_id.to_string(),
                            from.currency.unwrap(),
                            transaction.amount.amount.parse::<f64>().unwrap(),
                            0.0
                        )))
                    }
                } else if let Some(to) = transaction.to {
                    Ok(Box::new(CoinTransfer::new(
                        transaction.id,
                        transaction.created_at,
                        account_id.to_string(),
                        to.address.unwrap(),
                        to.currency.unwrap(),
                        transaction.amount.amount.parse::<f64>().unwrap(),
                        transaction.network.unwrap().transaction_fee.unwrap().amount.parse::<f64>().unwrap()
                    )))
                } else {
                    Err(CryptfolioError::CoinbaseAPIError("Failed to parse 'send' transaction".to_string()))
                }
            },
            "interest" => {
                // Incredibly small staking rewards will not give appropriate cost basis for asset
                let unit_price = match transaction.native_amount.amount.parse::<f64>().unwrap() < 0.01 {
                    true => { 0.0 },
                    false => { transaction.native_amount.amount.parse::<f64>().unwrap() / transaction.amount.amount.parse::<f64>().unwrap() },
                };
                Ok(Box::new(CoinReward::new(
                    transaction.id,
                    transaction.created_at,
                    transaction.amount.currency,
                    unit_price,
                    transaction.amount.amount.parse::<f64>().unwrap(),
                    "Staking".to_string(),
                    "Coinbase Rewards".to_string()
                )))
            },
            // "request" => {
                
            // },
            // "transfer" => {

            // },
            "buy" => {
                // Avoid redundant same-currency orders.
                if transaction.amount.currency == transaction.native_amount.currency {
                    Ok(Box::new(Dud {}))
                } else {
                    Ok(Box::new(CoinOrder::new(
                        transaction.id,
                        transaction.created_at,
                        format!("{}-USD", transaction.amount.currency),
                        transaction.buy.as_ref().unwrap().unit_price.as_ref().unwrap().amount.parse::<f64>().unwrap(),
                        transaction.amount.amount.parse::<f64>().unwrap(),
                        transaction.buy.as_ref().unwrap().fee.amount.parse::<f64>().unwrap(),
                        "buy".to_string(),
                        "Coinbase".to_string()
                    )))
                }
            },
            // "sell" => {
            //     Ok(Box::new(CoinOrder::new(
            //         transaction.id,
            //         transaction.created_at,
            //         format!("{}-USD", transaction.amount.currency),
            //         0.0,
            //         0.0,
            //         0.0,
            //         "side".to_string(),
            //         "platform".to_string()
            //     )))
            // },
            "fiat_deposit" => {
                Ok(Box::new(FiatTransfer::new(
                    transaction.id,
                    "deposit".to_string(),
                    "FIAT Institution".to_string(),
                    account_id.to_string(),
                    transaction.amount.amount.parse::<f64>().unwrap().abs()
                )))
            },
            // "fiat_withdrawal" => {
            // 
            // },
            "exchange_deposit" => {
                Ok(Box::new(FiatTransfer::new(
                    transaction.id,
                    "send".to_string(),
                    account_id.to_string(),
                    "Coinbase Pro".to_string(),
                    transaction.amount.amount.parse::<f64>().unwrap().abs()
                )))
            },
            "exchange_withdrawal" => {
                Ok(Box::new(FiatTransfer::new(
                    transaction.id,
                    "send".to_string(),
                    "Coinbase Pro".to_string(),
                    account_id.to_string(),
                    transaction.amount.amount.parse::<f64>().unwrap().abs()
                )))

            },
            "pro_deposit" => {
                Ok(Box::new(CoinTransfer::new(
                    transaction.id, 
                    transaction.created_at, 
                    account_id.to_string(), 
                    "Coinbase Pro".to_string(), 
                    transaction.amount.currency, 
                    transaction.amount.amount.parse::<f64>().unwrap(), 
                    0.0
                )))
            },
            "pro_withdrawal" => {
                Ok(Box::new(CoinTransfer::new(
                    transaction.id, 
                    transaction.created_at, 
                    "Coinbase Pro".to_string(), 
                    account_id.to_string(), 
                    transaction.amount.currency, 
                    transaction.amount.amount.parse::<f64>().unwrap(), 
                    0.0
                )))

            },
            // "vault_withdrawal" => {

            // },
            "advanced_trade_fill" => {
                // Avoid redundant same-currency orders.
                if transaction.amount.currency == transaction.native_amount.currency {
                    Ok(Box::new(Dud {}))
                } else {
                    let side: String;
                    let amount = transaction.amount.amount.parse::<f64>().unwrap();
                    if amount < 0.0 {
                        side = "sell".to_string();
                    } else {
                        side = "buy".to_string();
                    }

                    Ok(Box::new(CoinOrder::new(
                        transaction.id,
                        transaction.created_at,
                        format!("{}-USD", transaction.amount.currency),
                        transaction.advanced_trade_fill.as_ref().unwrap().fill_price.parse::<f64>().unwrap(),
                        transaction.amount.amount.parse::<f64>().unwrap().abs(),
                        transaction.native_amount.amount.parse::<f64>().unwrap().abs() - 
                        (transaction.amount.amount.parse::<f64>().unwrap().abs() *
                        transaction.advanced_trade_fill.as_ref().unwrap().fill_price.parse::<f64>().unwrap()),
                        side,
                        "Coinbase".to_string()
                    )))
                }
            },
            "trade" => {
                // clean this up...
                if let Some(resource) = trade_resources.get_mut(&transaction.trade.as_ref().unwrap().user_reference) {
                    if transaction.amount.amount.parse::<f64>().unwrap() < 0.0 {
                        resource.set_sell_side(
                            transaction.id,
                            transaction.amount.currency,
                            transaction.amount.amount.parse::<f64>().unwrap(),
                            transaction.native_amount.amount.parse::<f64>().unwrap()
                        );
                    } else {
                        resource.set_buy_side(
                            transaction.id,
                            transaction.amount.currency,
                            transaction.amount.amount.parse::<f64>().unwrap(), 
                            transaction.native_amount.amount.parse::<f64>().unwrap()
                        );
                    }
                } else {
                    trade_resources.insert(transaction.trade.as_ref().unwrap().user_reference.to_string(), TradeResource::new(transaction.created_at));
                    if transaction.amount.amount.parse::<f64>().unwrap() < 0.0 {
                        trade_resources.get_mut(&transaction.trade.as_ref().unwrap().user_reference).unwrap().set_sell_side(
                            transaction.id,
                            transaction.amount.currency,
                            transaction.amount.amount.parse::<f64>().unwrap(),
                            transaction.native_amount.amount.parse::<f64>().unwrap()
                        )
                    } else {
                        trade_resources.get_mut(&transaction.trade.as_ref().unwrap().user_reference).unwrap().set_buy_side(
                            transaction.id,
                            transaction.amount.currency,
                            transaction.amount.amount.parse::<f64>().unwrap(),
                            transaction.native_amount.amount.parse::<f64>().unwrap()
                        )
                    }
                }

                Ok(Box::new(Dud {}))
            },
            _ => {
                return Err(CryptfolioError::SyncError(format!("Unknown type of transaction '{}'", transaction.type_transaction)));
            }
        }
    }
}

#[async_trait]
impl SyncClient for Coinbase {
    async fn sync(&self) -> Result<Vec<Box<dyn DatabaseEntry + Send>>, CryptfolioError> {
        let mut result = Vec::<Box<dyn DatabaseEntry + Send>>::new();
        let mut trade_resources = HashMap::<String, TradeResource>::new();
        match self.client.fetch_accounts().await {
            Ok(response) => {
                for account in response {
                    // Avoid adding accounts which have not been initialized for use, thus no valid ID.
                    if account.id.len() < 10 {
                        continue;
                    }
                    result.push(Box::new(CoinAccount::new(
                        account.id.to_string(),
                        account.currency.code.to_string(),
                        "Coinbase".to_string()
                    )));
                    match self.client.fetch_account_transactions(&account.id).await {
                        Ok(response) => {
                            for transaction in response {
                                match self.process_transaction(&account.id, &mut trade_resources, transaction).await {
                                    Ok(db_entry) => {
                                        result.push(db_entry);
                                    },
                                    Err(e) => {
                                        return Err(CryptfolioError::CoinbaseAPIError(e.to_string()));
                                    }
                                }
                            }
                        },
                        Err(e) => {
                            return Err(CryptfolioError::CoinbaseAPIError(e.to_string()));
                        }
                    }
                }
            },
            Err(e) => {
                return Err(CryptfolioError::CoinbaseAPIError(e.to_string()));
            }
        }

        // clean-up any trade transactions and populate actual, correct queries for them into
        // 'result'
        for trade in trade_resources {
            if trade.1.is_valid() {
                result.push(Box::new(CoinOrder::new(
                    trade.1.buy_side_id,
                    trade.1.created_at.to_string(),
                    format!("{}-USD", trade.1.buy_side_currency),
                    trade.1.buy_side_native_amount / trade.1.buy_side_amount,
                    trade.1.buy_side_amount,
                    trade.1.sell_side_native_amount.abs() - trade.1.buy_side_native_amount,
                    "buy".to_string(),
                    "Coinbase".to_string()
                )));
                result.push(Box::new(CoinOrder::new(
                    trade.1.sell_side_id,
                    trade.1.created_at.to_string(),
                    format!("{}-USD", trade.1.sell_side_currency),
                    trade.1.sell_side_native_amount / trade.1.sell_side_amount,
                    trade.1.sell_side_amount,
                    0.0,
                    "sell".to_string(),
                    "Coinbase".to_string()
                )));
            }
        }

        Ok(result)

        // --------------------------------------------------------------------------------------------------

        // match self.client.fetch_accounts().await {
        //     Ok(response) => {
        //         for account in response {
        //             println!("id: {} | name: {}", account.id, account.currency.code);
        //         }
        //     },
        //     Err(e) => { return Err(CryptfolioError::CoinbaseAPIError(e.to_string())); }
        // }
        // Ok(())

        // match self.client.fetch_account_transactions(&"e7ccc453-7e87-50ee-94b9-7750c5e92935".to_string()).await {
        //     Ok(response) => {
        //         for transaction in response {
        //             match self.client.print_response(&format!(
        //                 "/v2/accounts/e7ccc453-7e87-50ee-94b9-7750c5e92935/transactions/{}?expand[]=buy&expand[]=sell&expand[]=trade",
        //                 transaction.id
        //             )).await {
        //                 Ok(response) => {
        //                     println!("{}", response);
        //                 } Err(e) => {
        //                     return Err(CryptfolioError::CoinbaseAPIError(e.to_string()));
        //                 }
        //             }
        //         }
        //     },
        //     Err(e) => {
        //         return Err(CryptfolioError::CoinbaseAPIError(e.to_string()));
        //     }
        // }
        // Ok(result)

        // match self.client.print_response(
        //     &"/v2/accounts/edc51d44-6f3a-5c59-9210-aff67ff6df83/transactions/9b5cd3c3-0a13-5504-9b0a-cd6337bd5f23?expand[]=buy&expand[]=sell&expand[]=trade"
        //     .to_string()
        // ).await {
        //     Ok(response) => { println!("{}", response); }
        //     Err(e) => { return Err(CryptfolioError::CoinbaseAPIError(e.to_string())); }
        // }
        // Ok(result)
    }
}

struct TradeResource {
    pub created_at: String,
    pub buy_side_id: String,
    pub buy_side_currency: String,
    pub buy_side_amount: f64,
    pub buy_side_native_amount: f64,
    pub sell_side_id: String,
    pub sell_side_currency: String,
    pub sell_side_amount: f64,
    pub sell_side_native_amount: f64,
}

impl TradeResource {
    pub fn new(created_at: String) -> TradeResource {
        TradeResource { 
           created_at: created_at, 
           buy_side_id: String::new(), 
           buy_side_currency: String::new(),
           buy_side_amount: 0.0, 
           buy_side_native_amount: 0.0, 
           sell_side_id: String::new(), 
           sell_side_currency: String::new(),
           sell_side_amount: 0.0, 
           sell_side_native_amount: 0.0 
        }
    }

    fn set_buy_side(&mut self, id: String, currency: String, amount: f64, native_amount: f64) {
        self.buy_side_id = id;
        self.buy_side_currency = currency;
        self.buy_side_amount = amount;
        self.buy_side_native_amount = native_amount;
    }

    fn set_sell_side(&mut self, id: String, currency: String, amount: f64, native_amount: f64) {
        self.sell_side_id = id;
        self.sell_side_currency = currency;
        self.sell_side_amount = amount;
        self.sell_side_native_amount = native_amount;
    }

    fn is_valid(&self) -> bool {
        if self.buy_side_amount != 0.0 && self.sell_side_amount != 0.0 {
            return true;
        }
        return false;
    }
}