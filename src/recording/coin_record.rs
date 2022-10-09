/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// coin_record.rs
/// 
/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// Description:
///     A structure that represents a record of a single coin.
/// 
/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// Usage:
///     Create an instance of a CoinRecord to query stored and calculated data regarding the asset.
/// 
///     let BTC: CoinRecord = CoinRecord::new("BTC").unwrap();
///     println!("You have {} BTC with an average cost of {} per share",
///         BTC.get_shares(), BTC.get_average_cost()
///     );
///     println!("Thus far, you have made ${} profit from BTC", BTC.get_profit());
/// 
/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// Notes:
///     Upon creation of a CoinRecord, information regarding the specified asset is pulled
///     from the database and any needed calculations are performed. This makes CoinRecord a 
///     snapshot at a particular time with the supplied data.
/// 
///     If any further data is to be added for an asset, a new CoinRecord should be created
///     to reflect changes.
/// 
/// ///////////////////////////////////////////////////////////////////////////////////////////////

use crate::recording::TaxRecord;
use std::collections::VecDeque;

struct TransactionRecord {
    date: String,
    price: f64,
    shares: f64,
    fee: f64,
}

pub struct CoinRecord {
    shares: f64,
    average_cost: f64,
    gross_profit: f64,
    total_invested: f64,
    current_invested: f64,
    total_fees: f64,
    buys: VecDeque<TransactionRecord>,
    sells: VecDeque<TransactionRecord>,
    tax_records: Vec<TaxRecord>,
}

impl CoinRecord {
    pub fn new() -> CoinRecord {
        CoinRecord { 
            shares: 0.0, 
            average_cost: 0.0, 
            gross_profit: 0.0, 
            total_invested: 0.0,
            current_invested: 0.0,
            total_fees: 0.0,
            buys: VecDeque::<TransactionRecord>::new(),
            sells: VecDeque::<TransactionRecord>::new(),
            tax_records: Vec::<TaxRecord>::new(),
        }
    }

    pub fn get_shares(&self) -> f64 {
        return self.shares;
    }

    pub fn get_average_cost(&self) -> f64 {
        return self.average_cost;
    }

    pub fn get_gross_profit(&self) -> f64 {
        return self.gross_profit;
    }

    pub fn get_net_profit(&self) -> f64 {
        return self.gross_profit - self.total_fees;
    }

    pub fn get_total_invested(&self) -> f64 {
        return self.total_invested;
    }

    pub fn get_current_invested(&self) -> f64 {
        return self.current_invested;
    }

    pub fn get_total_fees(&self) -> f64 {
        return self.total_fees;
    }

    pub fn get_tax_records(&self) -> &Vec<TaxRecord> {
        return &self.tax_records;
    }

    pub fn add_buy(&mut self, date: String, price: f64, shares: f64, fee: f64) {
        self.buys.push_back(TransactionRecord{date: date, price: price, shares: shares, fee: fee});
    }

    pub fn add_sell(&mut self, date: String, price: f64, shares: f64, fee: f64) {
        self.sells.push_back(TransactionRecord { date: date, price: price, shares: shares, fee: fee });
    }

    pub fn update(&mut self) {
        let mut i = 0;
        let mut j = self.buys.len();

        // calculate total fees (for buys)
        while i < j {
            self.total_fees += self.buys.get(i).unwrap().fee;
            i += 1;
        }

        // calculate profit
        j = self.sells.len();
        i = 0;
        while i < j {
            self.total_fees += self.sells.get(i).unwrap().fee;
            self.process_sell(i);
            i += 1;
        }
        self.sells.clear();

        // calculate remaining shares / avg cost / currenty invested / total invested
        let mut total_price = 0.0;
        for buy in &self.buys {
            total_price += buy.price * buy.shares;
            self.shares += buy.shares;
        }

        self.average_cost = total_price / self.shares;
        self.current_invested = total_price;
        self.total_invested += total_price;
    }

    fn process_sell(&mut self, index: usize) {
        let transaction = self.sells.get(index).unwrap();
        let mut remaining_sell_shares: f64 = transaction.shares;
        let mut i: usize = 0;
        let num_buys: usize = self.buys.len();
    
        while i < num_buys {
            let profit: f64;
            let current_buy_shares = self.buys.front().unwrap().shares;
            let current_buy_price = self.buys.front().unwrap().price;
            if current_buy_shares - remaining_sell_shares > 0.0 {
                // This particular buy order will still have shares remaining AFTER taking those
                // from this sell 
                profit = (remaining_sell_shares * transaction.price) - (remaining_sell_shares * current_buy_price);
                self.gross_profit += profit;
                self.total_invested += remaining_sell_shares * current_buy_price;
                self.buys.front_mut().unwrap().shares -= remaining_sell_shares;
                self.tax_records.push(TaxRecord::new(
                    self.buys.front().unwrap().date.to_string(),
                    transaction.date.to_string(),
                    current_buy_price,
                    transaction.price,
                    remaining_sell_shares,
                    profit
                ));
                break;
            } else {
                profit = (current_buy_shares * transaction.price) - (current_buy_shares * current_buy_price);
                self.gross_profit += profit;
                self.total_invested += current_buy_shares * current_buy_price;
                self.tax_records.push(TaxRecord::new(
                    self.buys.front().unwrap().date.to_string(),
                    transaction.date.to_string(),
                    current_buy_price,
                    transaction.price,
                    current_buy_shares,
                    profit
                ));
                self.buys.pop_front();
                remaining_sell_shares -= current_buy_shares;
            }
            i += 1;
        }
    }
}
