use std::fmt;

use crate::models::waypointtype;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
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

impl fmt::Display for ShipNavRouteWaypoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Sytem: {}({:?}), Coords {},{}",
            self.systemSymbol, self.type_field, self.x, self.y
        )
    }
}
