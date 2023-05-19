use std::default;

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
