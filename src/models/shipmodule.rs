use crate::models::shiprequirements;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub data: ShipModule,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
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
#[serde(rename_all = "camelCase")]
/**
 * A module can be installed in a ship and provides a set of capabilities such as storage space or quarters for crew. Module installations are permanent.
 */
pub struct ShipModule {
    pub symbol: ShipModuleSymbol,
    pub capacity: i64,
    pub range: i64,
    pub name: String,
    pub description: String,
    /**
     * The requirements for installation on a ship
     */
    pub requirements: shiprequirements::ShipRequirements,
}
