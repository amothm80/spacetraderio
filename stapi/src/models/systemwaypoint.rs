use std::fmt;

use crate::models::waypointtype;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
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

impl fmt::Display for SystemWaypoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let disp = format!(
            "Waypoint: {}({:?}) @ {},{}",
            self.symbol, self.type_field, self.x, self.y
        );

        write!(f, "{}", disp)
    }
}
