use crate::models::shiprequirements;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Data {
    pub data: ShipMount,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum ShipMountSymbol {
    #[default]
    MOUNT_GAS_SIPHON_I,
    MOUNT_GAS_SIPHON_II,
    MOUNT_GAS_SIPHON_III,
    MOUNT_SURVEYOR_I,
    MOUNT_SURVEYOR_II,
    MOUNT_SURVEYOR_III,
    MOUNT_SENSOR_ARRAY_I,
    MOUNT_SENSOR_ARRAY_II,
    MOUNT_SENSOR_ARRAY_III,
    MOUNT_MINING_LASER_I,
    MOUNT_MINING_LASER_II,
    MOUNT_MINING_LASER_III,
    MOUNT_LASER_CANNON_I,
    MOUNT_MISSILE_LAUNCHER_I,
    MOUNT_TURRET_I,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum ShipMountDeposit {
    #[default]
    QUARTZ_SAND,
    SILICON_CRYSTALS,
    PRECIOUS_STONES,
    ICE_WATER,
    AMMONIA_ICE,
    IRON_ORE,
    COPPER_ORE,
    SILVER_ORE,
    ALUMINUM_ORE,
    GOLD_ORE,
    PLATINUM_ORE,
    DIAMONDS,
    URANITE_ORE,
    MERITIUM_ORE,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
/**
 * A mount is installed on the exterier of a ship.
 */
pub struct ShipMount {
    pub symbol: ShipMountSymbol,
    pub name: String,
    pub description: String,
    pub strength: i64,
    pub deposits: ShipMountDeposit,
    /*** The requirements for installation on a ship  */
    pub requirements: shiprequirements::ShipRequirements,
}
