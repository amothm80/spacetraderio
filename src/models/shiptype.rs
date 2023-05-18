use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Data {
    pub data: ShipType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[allow(non_camel_case_types)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum ShipType {
    #[default]
    SHIP_PROBE,
    SHIP_MINING_DRONE,
    SHIP_INTERCEPTOR,
    SHIP_LIGHT_HAULER,
    SHIP_COMMAND_FRIGATE,
    SHIP_EXPLORER,
    SHIP_HEAVY_FREIGHTER,
    SHIP_LIGHT_SHUTTLE,
    SHIP_ORE_HOUND,
    SHIP_REFINING_FREIGHTER,
}
