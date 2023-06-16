use std::fmt;

use crate::models::systemfaction;
use crate::models::systemtype;
use crate::models::systemwaypoint;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct System {
    pub symbol: String,
    pub sectorSymbol: String,
    /**
     * The type of waypoint.
     */
    #[serde(rename = "type")]
    pub type_field: systemtype::SystemType,
    pub x: i64,
    pub y: i64,
    pub waypoints: Vec<systemwaypoint::SystemWaypoint>,
    pub factions: Vec<systemfaction::SystemFaction>,
}

impl fmt::Display for System {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut disp = format!(
            "System: {}\nSector: {}\nType: {:?}\nCoordinates: {},{}\n",
            self.symbol, self.sectorSymbol, self.type_field, self.x, self.y
        );

        for wp in &self.waypoints {
            disp = disp.to_owned() + format!("{}\n", wp).as_str();
        }

        for faction in &self.factions {
            disp = disp.to_owned() + format!("Faction: {}\n", faction.symbol).as_str();
        }

        write!(f, "{}", disp)
    }
}
