use crate::models::waypointtype;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub data: SystemWaypoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct SystemWaypoint {
    pub symbol: String,
    /**
     * The type of waypoint.
     */
    #[serde(rename = "type")]
    pub type_field: waypointtype::WaypointType,
    pub x: i64,
    pub y: i64,
}
