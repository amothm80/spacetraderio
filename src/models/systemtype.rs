use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
/**
 * The type of waypoint.
 */
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum SystemType {
    #[default]
    NEUTRON_STAR,
    RED_STAR,
    ORANGE_STAR,
    BLUE_STAR,
    YOUNG_STAR,
    WHITE_DWARF,
    BLACK_HOLE,
    HYPERGIANT,
    NEBULA,
    UNSTABLE,
}
