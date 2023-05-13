use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub data: ShipNavFlightMode,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/**
 * The ship's set speed when traveling between waypoints or systems.
 */
pub enum ShipNavFlightMode {
    #[default]
    DRIFT,
    STEALTH,
    CRUISE,
    BURN,
}
