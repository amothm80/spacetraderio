use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub data: ShipyardTransaction,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

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
