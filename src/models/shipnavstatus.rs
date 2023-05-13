use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub data: ShipNavStatus,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
/**
 * The current status of the ship
 */
pub enum ShipNavStatus {
    #[default]
    IN_TRANSIT,
    IN_ORBIT,
    DOCKED,
}
