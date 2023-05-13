use crate::models::shipcondition;
use crate::models::shiprequirements;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub data: ShipEngine,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum ShipEngineSymbol {
    #[default]
    ENGINE_IMPULSE_DRIVE_I,
    ENGINE_ION_DRIVE_I,
    ENGINE_ION_DRIVE_II,
    ENGINE_HYPER_DRIVE_I,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/**
 * The engine determines how quickly a ship travels between waypoints.
 */
pub struct ShipEngine {
    pub symbol: ShipEngineSymbol,
    pub name: String,
    pub description: String,
    /**
     * Condition is a range of 0 to 100 where 0 is completely worn out and 100 is brand new.
     */
    pub condition: shipcondition::ShipCondition,
    pub speed: i64,
    /**
     * The requirements for installation on a ship
     */
    pub requirements: shiprequirements::ShipRequirements,
}
