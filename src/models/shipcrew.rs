use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Data {
    pub data: ShipCrew,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum ShipCrewRotation {
    STRICT,
    #[default]
    RELAXED,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
/**
 * The ship's crew service and maintain the ship's systems and equipment.
 */
pub struct ShipCrew {
    /**
     * The current i64, of crew members on the ship.
     */
    pub current: i64,
    /**
     * The minimum i64, of crew members required to maintain the ship.
     */
    pub required: i64,
    /**
     * The maximum i64, of crew members the ship can support.
     */
    pub capacity: i64,
    /**
     * The rotation of crew shifts. A stricter shift improves the ship's performance. A more relaxed shift improves the crew's morale.
     */
    pub rotation: ShipCrewRotation,
    /**
     * A rough measure of the crew's morale. A higher morale means the crew is happier and more productive. A lower morale means the ship is more prone to accidents.
     */
    pub morale: i64,
    /**
     * The amount of credits per crew member paid per hour. Wages are paid when a ship docks at a civilized waypoint.
     */
    pub wages: i64,
}
