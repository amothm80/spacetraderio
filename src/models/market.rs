use std::fmt;

use crate::models::markettradegood;
use crate::models::markettransaction;
use crate::models::tradegood;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Market {
    /**
     * The symbol of the market. The symbol is the same as the waypoint where the market is located.
     */
    pub symbol: String,
    /**
     * The list of goods that are exported from this market.
     */
    pub exports: Vec<tradegood::TradeGood>,
    /**
     * The list of goods that are sought as imports in this market.
     */
    pub imports: Vec<tradegood::TradeGood>,
    /**
     * The list of goods that are bought and sold between agents at this market.
     */
    pub exchange: Vec<tradegood::TradeGood>,
    /**
     * The list of recent transactions at this market. Visible only when a ship is present at the market.
     */
    #[serde(default)]
    pub transactions: Vec<markettransaction::MarketTransaction>,
    /**
     * The list of goods that are traded at this market. Visible only when a ship is present at the market.
     */
    #[serde(default)]
    pub tradeGoods: Vec<markettradegood::MarketTradeGood>,
}

impl fmt::Display for Market {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut disp = format!("Symbol: {}\n", self.symbol);
        let mut count = 1;

        if !self.exports.is_empty() {
            disp = disp.to_owned() + "Exports:\n";
            for export in &self.exports {
                disp = disp.to_owned() + format!("{} - {}", count, export).as_str();
                count += 1;
            }
        }
        disp = disp.to_owned() + "\n";
        count = 1;

        if !self.imports.is_empty() {
            disp = disp.to_owned() + "Imports:\n";
            for import in &self.imports {
                disp = disp.to_owned() + format!("{} - {}", count, import).as_str();
                count += 1;
            }
        }
        disp = disp.to_owned() + "\n";
        count = 1;

        if !self.exchange.is_empty() {
            disp = disp.to_owned() + "Exchanges:\n";
            for ex in &self.exchange {
                disp = disp.to_owned() + format!("{} - {}", count, ex).as_str();
                count += 1;
            }
        }
        disp = disp.to_owned() + "\n";

        if !self.transactions.is_empty() {
            disp = disp.to_owned() + "Transactions:\n";
            for tx in &self.transactions {
                disp = disp.to_owned() + format!("{}\n", tx).as_str();
            }
        }

        if !self.tradeGoods.is_empty() {
            disp = disp.to_owned() + "Trade Goods:\n";
            for td in &self.tradeGoods {
                disp = disp.to_owned() + format!("{}\n", td).as_str();
            }
        }

        write!(f, "{}", disp)
    }
}
