use serde_derive::Deserialize;
use serde_derive::Serialize;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
/**
 * The ship's set speed when traveling between waypoints or systems.
 */
pub enum ShipNavFlightMode {
    DRIFT,
    STEALTH,
    #[serde(rename = "CRUISE")]
    #[default]
    CRUISE,
    BURN,
}
