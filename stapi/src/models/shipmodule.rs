use std::fmt;

use crate::models::shiprequirements;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum ShipModuleSymbol {
    #[default]
    MODULE_MINERAL_PROCESSOR_I,
    MODULE_CARGO_HOLD_I,
    MODULE_CREW_QUARTERS_I,
    MODULE_ENVOY_QUARTERS_I,
    MODULE_PASSENGER_CABIN_I,
    MODULE_MICRO_REFINERY_I,
    MODULE_ORE_REFINERY_I,
    MODULE_FUEL_REFINERY_I,
    MODULE_SCIENCE_LAB_I,
    MODULE_JUMP_DRIVE_I,
    MODULE_JUMP_DRIVE_II,
    MODULE_JUMP_DRIVE_III,
    MODULE_WARP_DRIVE_I,
    MODULE_WARP_DRIVE_II,
    MODULE_WARP_DRIVE_III,
    MODULE_SHIELD_GENERATOR_I,
    MODULE_SHIELD_GENERATOR_II,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
/**
 * A module can be installed in a ship and provides a set of capabilities such as storage space or quarters for crew. Module installations are permanent.
 */
pub struct ShipModule {
    pub symbol: ShipModuleSymbol,
    #[serde(default)]
    pub capacity: i64,
    #[serde(default)]
    pub range: i64,
    pub name: String,
    #[serde(default)]
    pub description: String,
    /**
     * The requirements for installation on a ship
     */
    pub requirements: shiprequirements::ShipRequirements,
}

impl fmt::Display for ShipModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut disp = format!(
            "{}({:?}) - RANGE: {} - CAP: {}\n",
            self.name, self.symbol, self.range, self.capacity
        );
        disp = disp.to_owned() + format!("Requirements: {}\n", self.requirements).as_str();

        write!(f, "{}", disp)
    }
}
