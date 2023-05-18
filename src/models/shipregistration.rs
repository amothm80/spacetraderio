use crate::models::shiprole;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Data {
    pub data: ShipRegistration,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
/**
 * The public registration information of the ship
 */
pub struct ShipRegistration {
    /**
     * The agent's registered name of the ship
     */
    pub name: String,
    /**
     * The symbol of the faction the ship is registered with
     */
    pub factionSymbol: String,
    /**
     * The registered role of the ship
     */
    pub role: shiprole::ShipRole,
}
