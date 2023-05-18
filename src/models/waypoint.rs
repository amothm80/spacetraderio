use crate::models::chart;
use crate::models::waypointfaction;
use crate::models::waypointorbital;
use crate::models::waypointtrait;
use crate::models::waypointtype;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Data {
    pub data: Waypoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
/**
 * A waypoint is a location that ships can travel to such as a Planet, Moon or Space Station.
 */
pub struct Waypoint {
    symbol: String,
    /**
     * The type of waypoint.
     */
    #[serde(rename = "type")]
    type_field: waypointtype::WaypointType,
    systemSymbol: String,
    x: i64,
    y: i64,
    orbitals: Vec<waypointorbital::WaypointOrbital>,
    faction: waypointfaction::WaypointFaction,
    /**
     * The traits of the waypoint.
     */
    traits: Vec<waypointtrait::WaypointTrait>,
    /**
     * The chart of a system or waypoint, which makes the location visible to other agents.
     */
    chart: chart::Chart,
}
