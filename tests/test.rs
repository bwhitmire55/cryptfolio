use cryptfolio::app::CryptfolioApp;

// #[tokio::test]
// async fn coinbase_sync() {
//     let app = CryptfolioApp::new("local/test.db").unwrap();
//     let platform = cryptfolio::platform::exchange::Coinbase::new(
//         "KEY",
//         "SECRET"
//     ).unwrap();
//     let handle = app.add_platform("Default profile", platform).unwrap();

//     app.sync_platform(handle).await.unwrap();
// }

// #[tokio::test]
// async fn coinbase_pro_sync() {
//     let app = CryptfolioApp::new("local/cb_pro.db").unwrap();
//     let default_profile = cryptfolio::platform::exchange::CoinbasePro::new(
//         "KEY",
//         "SECRET",
//         "PASSPHRASE"
//     ).unwrap();
//     let blake_profile = cryptfolio::platform::exchange::CoinbasePro::new(
//         "KEY",
//         "SECRET",
//         "PASSPHRASE"
//     ).unwrap();
//     let handle = app.add_platform("Default profile", default_profile).unwrap();
//     let handle2 = app.add_platform("Blake profile", blake_profile).unwrap();

//     app.sync_platform(handle).await.unwrap();
//     app.sync_platform(handle2).await.unwrap();
// }

// #[tokio::test(flavor = "multi_thread")]
// async fn solana_sync() {
//     let app = CryptfolioApp::new("local/test.db").unwrap();
//     let solflare = cryptfolio::platform::blockchain::Solana::new(
//         "ADDRESS".to_string()
//     ).unwrap();

//     let handle = app.add_platform("Solflare Wallet", solflare).unwrap();
    
//     app.sync_platform(handle).await.unwrap();
// }

// #[test]
// fn coin_record() {
//     let app = CryptfolioApp::new("local/test.db").unwrap();
//     let coin = app.get_coin_record("LINK".to_string());

//     println!("Balance: {} | Avg: {} | Gross: {} | Net: {} | Current Invested: {} | Total Invested: {} | Fees: {}",
//         coin.get_shares(),
//         coin.get_average_cost(),
//         coin.get_gross_profit(),
//         coin.get_net_profit(),
//         coin.get_current_invested(),
//         coin.get_total_invested(),
//         coin.get_total_fees()
//     );

//     for record in coin.get_tax_records() {
//         println!("{:#?}", record);
//     }
// }

// #[test]
// fn platform_connection_persistance() {
//     let app = CryptfolioApp::new("local/test.db").unwrap();
//     let coinbase = cryptfolio::platform::exchange::Coinbase::new("KEY", "SECRET").unwrap();
//     let coinbase_pro_1 = cryptfolio::platform::exchange::CoinbasePro::new("KEY", "SECRET", "PASSPHRASE").unwrap();
//     let coinbase_pro_2 = cryptfolio::platform::exchange::CoinbasePro::new("KEY", "SECRET", "PASSPHRASE").unwrap();
//     app.add_platform("Default Portfolio", coinbase).unwrap();
//     app.add_platform("Default Portfolio", coinbase_pro_1).unwrap();
//     app.add_platform("Blake", coinbase_pro_2).unwrap();
// }

#[test]
fn get_connections() {
    let app = CryptfolioApp::new("local/test.db").unwrap();
    app.get_connections();
}