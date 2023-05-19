use serde_derive::Deserialize;
use serde_derive::Serialize;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
/**
 * The current status of the ship
 */
pub enum ShipNavStatus {
    IN_TRANSIT,
    IN_ORBIT,
    #[serde(rename = "DOCKED")]
    #[default]
    DOCKED,
}
