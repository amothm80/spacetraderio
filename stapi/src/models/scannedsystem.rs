use std::fmt;

use crate::models::systemtype;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct ScannedSystem {
    pub symbol: String,
    pub sectorSymbol: String,
    /**
     * The type of waypoint.
     */
    #[serde(rename = "type")]
    pub type_field: systemtype::SystemType,
    pub x: i64,
    pub y: i64,
    pub distance: i64,
}

impl fmt::Display for ScannedSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Symbol: {}\nSector Symbol: {}\nType: {:#?}\nCoordinates: {},{}\nDistance: {}",
            self.symbol, self.sectorSymbol, self.type_field, self.x, self.y, self.distance
        )
    }
}
