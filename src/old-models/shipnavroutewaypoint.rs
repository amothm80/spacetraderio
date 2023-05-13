use crate::models::waypointtype;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub data: ShipNavRouteWaypoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/**
 * The destination or departure of a ships nav route.
 */
pub struct ShipNavRouteWaypoint {
    pub symbol: String,
    /**
     * The type of waypoint.
     */
    #[serde(rename = "type")]
    pub type_field: waypointtype::WaypointType,
    pub systemSymbol: String,
    pub x: i64,
    pub y: i64,
}
