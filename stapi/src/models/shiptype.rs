use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[allow(non_camel_case_types)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum ShipTypeEnum {
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
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[allow(non_camel_case_types)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct ShipType {
    #[serde(rename = "type")]
    type_field: ShipTypeEnum,
}

impl fmt::Display for ShipType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let disp = format!("{:?}", self.type_field);

        write!(f, "{}", disp)
    }
}
