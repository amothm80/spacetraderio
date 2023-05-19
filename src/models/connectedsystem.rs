use crate::models::systemtype;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct ConntectedSystem {
    pub symbol: String,
    pub sectorSymbol: String,
    /**
     * The type of waypoint.
     */
    #[serde(rename = "type")]
    pub type_field: systemtype::SystemType,
    /**
     * The symbol of the faction that owns the connected jump gate in the system.
     */
    #[serde(default)]
    pub factionSymbol: String,
    pub x: i64,
    pub y: i64,
    pub distance: i64,
}
