/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// solana.rs
/// 
/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// Description:
///     Solana (Blockchain) implementation for syncing transaction history.
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
use crate::database::entry::DatabaseEntry;
use crate::platform::SyncClient;
use async_trait::async_trait;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_sdk::signature::Signature;
use solana_transaction_status::UiTransactionEncoding;
use solana_transaction_status::UiTransactionStatusMeta;
use std::str::FromStr;

pub struct Solana { 
    wallet: String,
    client: RpcClient,
}

impl Solana {
    pub fn new(wallet: String) -> Result<Solana, CryptfolioError> {
        Ok(
            Solana {
                wallet: wallet,
                client: RpcClient::new("https://api.mainnet-beta.solana.com"),
            }
        )
    }

    pub fn get_wallet(&self) -> String {
        self.wallet.to_string()
    }

    // fn process_transaction(&self, transaction: UiTransactionStatusMeta) -> Box<dyn DatabaseEntry + Send> {

    // }

    fn process_transaction(&self, transaction: UiTransactionStatusMeta) {
        println!("Amount: {}", (transaction.pre_balances[0].wrapping_sub(transaction.post_balances[0])) / LAMPORTS_PER_SOL);
        println!("Fee: {}", transaction.fee / LAMPORTS_PER_SOL);
    }
}

#[async_trait]
impl SyncClient for Solana {
    async fn sync(&self) -> Result<Vec<Box<dyn DatabaseEntry + Send>>, CryptfolioError> {
        let result = Vec::<Box<dyn DatabaseEntry + Send>>::new();

        // let config = GetConfirmedSignaturesForAddress2Config {
        //     before: None,
        //     until: None,
        //     limit: Some(3),
        //     commitment: Some(CommitmentConfig::confirmed()),
        // };

        match self.client.get_signatures_for_address(&Pubkey::from_str(self.wallet.as_str()).unwrap()) {
            Ok(signatures) => {
                for signature in signatures {
                    match self.client.get_transaction(&Signature::from_str(signature.signature.as_str()).unwrap(), UiTransactionEncoding::Json) {
                        Ok(transaction) => {
                            // result.push(self.process_transaction(transaction.transaction.meta.unwrap()));
                            self.process_transaction(transaction.transaction.meta.unwrap());
                        }
                        Err(e) => {
                            return Err(CryptfolioError::SolanaAPIError(e.to_string()));
                        }
                    }
                }
            },
            Err(e) => {
                return Err(CryptfolioError::SolanaAPIError(e.to_string()));
            }
        }

        Ok(result)
    }
}