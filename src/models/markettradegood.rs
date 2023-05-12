use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct Data{
    pub data: MarketTradeGood,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MarketTradeGoodSupply{
    SCARCE,
    LIMITED,
    #[default]
    MODERATE,
    ABUNDANT,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketTradeGood {
    /**
     * The symbol of the trade good.
     */
    pub symbol: String,
    /**
     * The typical volume flowing through the market for this type of good. The larger the trade volume, the more stable prices will be.
     */
    pub tradeVolume: i64,
    /**
     * A rough estimate of the total supply of this good in the marketplace.
     */
    pub supply: MarketTradeGoodSupply,
    /**
     * The price at which this good can be purchased from the market.
     */
    pub purchasePrice: i64,
    /**
     * The price at which this good can be sold to the market.
     */
    pub sellPrice: i64,
  }
  