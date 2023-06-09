use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
/**
 * An orbital is another waypoint that orbits a parent waypoint.
 */
pub struct WaypointOrbital {
    pub symbol: String,
}

impl fmt::Display for WaypointOrbital {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Orbital: {}", self.symbol)
    }
}
