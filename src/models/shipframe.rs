use std::fmt;

use crate::models::shipcondition;
use crate::models::shiprequirements;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum ShipFrameSymbol {
    #[default]
    FRAME_PROBE,
    FRAME_DRONE,
    FRAME_INTERCEPTOR,
    FRAME_RACER,
    FRAME_FIGHTER,
    #[serde(rename = "FRAME_FRIGATE")]
    FRAME_FRIGATE,
    FRAME_SHUTTLE,
    FRAME_EXPLORER,
    FRAME_MINER,
    FRAME_LIGHT_FREIGHTER,
    FRAME_HEAVY_FREIGHTER,
    FRAME_TRANSPORT,
    FRAME_DESTROYER,
    FRAME_CRUISER,
    FRAME_CARRIER,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
/**
 * The frame of the ship. The frame determines the i64, of modules and mounting points of the ship, as well as base fuel capacity. As the condition of the frame takes more wear, the ship will become more sluggish and less maneuverable.
 */
pub struct ShipFrame {
    pub symbol: ShipFrameSymbol,
    pub name: String,
    pub description: String,
    /**
     * Condition is a range of 0 to 100 where 0 is completely worn out and 100 is brand new.
     */
    #[serde(default)]
    pub condition: shipcondition::ShipCondition,
    pub moduleSlots: i64,
    pub mountingPoints: i64,
    pub fuelCapacity: i64,
    /**
     * The requirements for installation on a ship
     */
    pub requirements: shiprequirements::ShipRequirements,
}

impl fmt::Display for ShipFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({:?})\nCondition: {}\nModule Slots: {}\nMounting Points: {}\nFuel Capacity:{}\nRequirements: {}",
            self.name, self.symbol, self.condition, self.moduleSlots, self.mountingPoints, self.fuelCapacity, self.requirements
        )
    }
}
