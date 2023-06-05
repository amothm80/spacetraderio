use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

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
    #[serde(default)]
    pub power: i64,
    /**
     * The i64, of crew required for operation.
     */
    #[serde(default)]
    pub crew: i64,
    /**
     * The i64, of module slots required for installation.
     */
    #[serde(default)]
    pub slots: i64,
}

impl fmt::Display for ShipRequirements {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "POWER: {} - CREW: {} - SLOTS: {}",
            self.power, self.crew, self.slots
        )
    }
}
