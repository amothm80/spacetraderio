use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct ShipyardTransaction {
    /**
     * The symbol of the waypoint where the transaction took place.
     */
    pub waypointSymbol: String,
    /**
     * The symbol of the ship that was purchased.
     */
    pub shipSymbol: String,
    /**
     * The price of the transaction.
     */
    pub price: i64,
    /**
     * The symbol of the agent that made the transaction.
     */
    pub agentSymbol: String,
    /**
     * The timestamp of the transaction.
     */
    pub timestamp: String,
}

impl fmt::Display for ShipyardTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let disp = format!(
            "Transaction waypoint: {}\n
            Ship bought: {}\n
            Ship price: {}\n
            Bought by: {}\n
            On: {}\n",
            self.waypointSymbol, self.shipSymbol, self.price, self.agentSymbol, self.timestamp
        );

        write!(f, "{}", disp)
    }
}
