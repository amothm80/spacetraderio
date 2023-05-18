use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Data {
    pub data: ShipRequirements,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
/**
 * The requirements for installation on a ship
 */
pub struct ShipRequirements {
    /**
     * The amount of power required from the reactor.
     */
    pub power: i64,
    /**
     * The i64, of crew required for operation.
     */
    pub crew: i64,
    /**
     * The i64, of module slots required for installation.
     */
    pub slots: i64,
}
