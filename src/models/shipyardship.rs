use crate::models::shipengine;
use crate::models::shipframe;
use crate::models::shipmodule;
use crate::models::shipmount;
use crate::models::shipreactor;
use crate::models::shiptype;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct ShipyardShip {
    #[serde(default)]
    pub symbol: String,
    #[serde(rename = "type")]
    pub type_field: shiptype::ShipTypeEnum,
    pub name: String,
    pub description: String,
    pub purchasePrice: i64,
    /**
     * The frame of the ship. The frame determines the number of modules and mounting points of the ship, as well as base fuel capacity. As the condition of the frame takes more wear, the ship will become more sluggish and less maneuverable.
     */
    pub frame: shipframe::ShipFrame,
    /**
     * The reactor of the ship. The reactor is responsible for powering the ship's systems and weapons.
     */
    pub reactor: shipreactor::ShipReactor,
    /**
     * The engine determines how quickly a ship travels between waypoints.
     */
    pub engine: shipengine::ShipEngine,
    pub modules: Vec<shipmodule::ShipModule>,
    pub mounts: Vec<shipmount::ShipMount>,
}
