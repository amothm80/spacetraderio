use std::fmt;

use crate::models::shipcondition;
use crate::models::shiprequirements;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum ShipReactorSymbol {
    #[default]
    REACTOR_SOLAR_I,
    REACTOR_FUSION_I,
    #[serde(rename = "REACTOR_FISSION_I")]
    REACTOR_FISSION_I,
    REACTOR_CHEMICAL_I,
    REACTOR_ANTIMATTER_I,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
/**
 * The reactor of the ship. The reactor is responsible for powering the ship's systems and weapons.
 */
pub struct ShipReactor {
    pub symbol: ShipReactorSymbol,

    pub name: String,
    pub description: String,
    /**
     * Condition is a range of 0 to 100 where 0 is completely worn out and 100 is brand new.
     */
    #[serde(default)]
    pub condition: shipcondition::ShipCondition,
    pub powerOutput: i64,
    /**
     * The requirements for installation on a ship
     */
    pub requirements: shiprequirements::ShipRequirements,
}

impl fmt::Display for ShipReactor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({:?})\nCondition: {}\nPower Output: {}\nRequirements: {}",
            self.name, self.symbol, self.condition, self.powerOutput, self.requirements
        )
    }
}
