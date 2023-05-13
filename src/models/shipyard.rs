use crate::models::shiptype;
use crate::models::shipyardship;
use crate::models::shipyardtransaction;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub data: Shipyard,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct Shipyard {
    /**
     * The symbol of the shipyard. The symbol is the same as the waypoint where the shipyard is located.
     */
    pub symbol: String,
    /**
     * The list of ship types available for purchase at this shipyard.
     */
    pub shipTypes: Vec<shiptype::ShipType>,
    /**
     * The list of recent transactions at this shipyard.
     */
    pub transactions: Vec<shipyardtransaction::ShipyardTransaction>,
    /**
     * The ships that are currently available for purchase at the shipyard.
     */
    pub ships: Vec<shipyardship::ShipyardShip>,
}
