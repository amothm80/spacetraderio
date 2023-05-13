use crate::models::systemfaction;
use crate::models::systemtype;
use crate::models::systemwaypoint;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub data: System,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct System {
    pub symbol: String,
    pub sectorSymbol: String,
    /**
     * The type of waypoint.
     */
    #[serde(rename = "type")]
    pub type_field: systemtype::SystemType,
    pub x: i64,
    pub y: i64,
    pub waypoints: Vec<systemwaypoint::SystemWaypoint>,
    pub factions: Vec<systemfaction::SystemFaction>,
}
