use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
/**
 * The type of waypoint.
 */
pub enum WaypointType {
    #[default]
    PLANET,
    GAS_GIANT,
    MOON,
    ORBITAL_STATION,
    JUMP_GATE,
    ASTEROID_FIELD,
    NEBULA,
    DEBRIS_FIELD,
    GRAVITY_WELL,
}
