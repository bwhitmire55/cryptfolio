/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// tax_record.rs
/// 
/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// Description:
///     A structure that represents a record of a single taxable event.
/// 
/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// Usage:
///     Create an instance of a TaxRecord to retrieve information regarding the taxable event.
/// 
///     let BTC: CoinRecord = CoinRecord::new("BTC").unwrap();
///     for record in BTC.get_tax_records() {
///         println!("BTC {}: {} | {} | {}",
///             record.get_sell_date(),
///             record.get_profit(),
///             record.get_tax_obligation_type().to_string(),
///             record.get_tax_obligation()
///         );
///     }
/// 
/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// Notes:
/// 
/// ///////////////////////////////////////////////////////////////////////////////////////////////

use crate::util::Util;

#[derive(Debug)]
pub enum TaxObligationType {
    ShortTermCapitalGains,
    LongTermCapitalGains,
}

impl TaxObligationType {
    pub fn to_percentage(&self) -> f64 {
        match self {
            Self::ShortTermCapitalGains => {
                return 0.22;
            },
            Self::LongTermCapitalGains => {
                return 0.15;
            }
        }
    }
}

#[derive(Debug)]
pub struct TaxRecord {
    pub buy_date: String,
    pub sell_date: String,
    pub buy_price: f64,
    pub sell_price: f64,
    pub unit_size: f64,
    pub tax_obligation_type: TaxObligationType,
    pub profit: f64,
    pub tax_obligation: f64,
}

impl TaxRecord {
    pub fn new(buy_date: String, sell_date: String, buy_price: f64, sell_price: f64, unit_size: f64, profit: f64) -> TaxRecord {
        let ob_type = match (Util::parse_date(sell_date.as_str()).unwrap() - Util::parse_date(buy_date.as_str()).unwrap()).num_days() < 365 {
            true => { TaxObligationType::ShortTermCapitalGains },
            false => { TaxObligationType::LongTermCapitalGains }
        };

        let ob = profit * ob_type.to_percentage();
        
        TaxRecord { 
            buy_date: buy_date,
            sell_date: sell_date, 
            buy_price: buy_price, 
            sell_price: sell_price, 
            unit_size: unit_size,
            tax_obligation_type: ob_type, 
            profit: profit,
            tax_obligation: ob,
        }
    }
}