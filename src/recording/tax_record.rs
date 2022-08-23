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
    pub tax_obligation_type: TaxObligationType,
    pub profit: f64,
    pub tax_obligation: f64,
}

impl TaxRecord {
    pub fn new(buy_date: String, sell_date: String, buy_price: f64, sell_price: f64, profit: f64) -> TaxRecord {
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
            tax_obligation_type: ob_type, 
            profit: profit,
            tax_obligation: ob,
        }
    }
}