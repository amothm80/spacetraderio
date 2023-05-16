use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub data: ShipCargoItem,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/**
 * The type of cargo item and the number of units.
 */
pub struct ShipCargoItem {
    /**
     * The unique identifier of the cargo item type.
     */
    pub symbol: String,
    /**
     * The name of the cargo item type.
     */
    pub name: String,
    /**
     * The description of the cargo item type.
     */
    pub description: String,
    /**
     * The number of units of the cargo item.
     */
    pub units: i64,
}