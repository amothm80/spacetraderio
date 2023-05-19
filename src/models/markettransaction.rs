use serde_derive::Deserialize;
use serde_derive::Serialize;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum MarketTransactionType {
    #[default]
    PURCHASE,
    SELL,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MarketTransaction {
    /**
     * The symbol of the waypoint where the transaction took place.
     */
    pub waypointSymbol: String,
    /**
     * The symbol of the ship that made the transaction.
     */
    pub shipSymbol: String,
    /**
     * The symbol of the trade good.
     */
    pub tradeSymbol: String,
    /**
     * The type of transaction.
     */
    #[serde(rename = "type")]
    pub type_field: MarketTransactionType,
    /**
     * The number of units of the transaction.
     */
    pub units: i64,
    /**
     * The price per unit of the transaction.
     */
    pub pricePerUnit: i64,
    /**
     * The total price of the transaction.
     */
    pub totalPrice: i64,
    /**
     * The timestamp of the transaction.
     */
    pub timestamp: String,
}
