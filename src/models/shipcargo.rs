use crate::models::shipcargoitem;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Data {
    pub data: ShipCargo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct ShipCargo {
    /**
     * The max i64, of items that can be stored in the cargo hold.
     */
    pub capacity: i64,
    /**
     * The i64, of items currently stored in the cargo hold.
     */
    pub units: i64,
    /**
     * The items currently in the cargo hold.
     */
    pub inventory: Vec<shipcargoitem::ShipCargoItem>,
}
