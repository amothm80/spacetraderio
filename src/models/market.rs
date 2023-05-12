use serde_derive::Deserialize;
use serde_derive::Serialize;
use crate::models::markettradegood;
use crate::models::markettransaction

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct Data{
    pub data: Market,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    /**
     * The symbol of the market. The symbol is the same as the waypoint where the market is located.
     */
    pub symbol: String,
    /**
     * The list of goods that are exported from this market.
     */
    pub exports: Vec<tradegood>,
    /**
     * The list of goods that are sought as imports in this market.
     */
    pub imports: Vec<tradegood>,
    /**
     * The list of goods that are bought and sold between agents at this market.
     */
    pub exchange: Vec<tradegood>,
    /**
     * The list of recent transactions at this market. Visible only when a ship is present at the market.
     */
    pub transactions: Vec<markettransaction::MarketTransaction>,
    /**
     * The list of goods that are traded at this market. Visible only when a ship is present at the market.
     */
    pub tradeGoods: Vec<markettradegood::MarketTradeGood>,
  }
  