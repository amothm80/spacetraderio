use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum MarketTradeGoodSupply {
    SCARCE,
    LIMITED,
    #[default]
    MODERATE,
    ABUNDANT,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
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

impl fmt::Display for MarketTradeGood {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut disp = format!(
            "Symbol: {}\nTrade Volume: {}\nSupply: {:?}\nPurchase Price: {}\nSell Price: {}\n",
            self.symbol, self.tradeVolume, self.supply, self.purchasePrice, self.sellPrice
        );

        write!(f, "{}", disp)
    }
}
