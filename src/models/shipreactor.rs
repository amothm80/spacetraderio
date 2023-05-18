use crate::models::shipcondition;
use crate::models::shiprequirements;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Data {
    pub data: ShipReactor,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum ShipReactorSymbol {
    #[default]
    REACTOR_SOLAR_I,
    REACTOR_FUSION_I,
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
    pub condition: shipcondition::ShipCondition,
    pub powerOutput: i64,
    /**
     * The requirements for installation on a ship
     */
    pub requirements: shiprequirements::ShipRequirements,
}
