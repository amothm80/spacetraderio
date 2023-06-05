use std::default;
use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

/**
 * The chart of a system or waypoint, which makes the location visible to other agents.
 */
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Chart {
    #[serde(default)]
    pub waypointSymbol: String,
    #[serde(default)]
    pub submittedBy: String,
    #[serde(default)]
    pub submittedOn: String,
}

impl fmt::Display for Chart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Chart Information:\nWaypoint: {}\nSubmitted By: {}\nSubmitted On: {}\n",
            self.waypointSymbol, self.submittedBy, self.submittedOn
        )
    }
}
