use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub data: SystemType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
/**
 * The type of waypoint.
 */
#[allow(non_camel_case_types)]
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