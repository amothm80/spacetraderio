use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct Data{
    pub data: Chart,
}

/**
 * The chart of a system or waypoint, which makes the location visible to other agents.
 */
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    pub waypointSymbol: String,
    pub submittedBy: String,
    pub submittedOn: String,
  }
  