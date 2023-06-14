use std::fmt;

use crate::models::chart;
use crate::models::waypointfaction;
use crate::models::waypointorbital;
use crate::models::waypointtrait;
use crate::models::waypointtype;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
/**
 * A waypoint is a location that ships can travel to such as a Planet, Moon or Space Station.
 */
pub struct ScannedWaypoint {
    pub symbol: String,
    /**
     * The type of waypoint.
     */
    #[serde(rename = "type")]
    pub type_field: waypointtype::WaypointType,
    pub systemSymbol: String,
    pub x: i64,
    pub y: i64,
    pub orbitals: Vec<waypointorbital::WaypointOrbital>,
    #[serde(default)]
    pub faction: waypointfaction::WaypointFaction,
    /**
     * The traits of the waypoint.
     */
    pub traits: Vec<waypointtrait::WaypointTrait>,
    /**
     * The chart of a system or waypoint, which makes the location visible to other agents.
     */
    #[serde(default)]
    pub chart: chart::Chart,
}

impl fmt::Display for ScannedWaypoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let faction_check = waypointfaction::WaypointFaction::default();
        let char_check = chart::Chart::default();
        let mut disp = format!(
            "Symbol: {}\nSystem Symbol: {}\nType: {:#?}\nCoordinates: {},{}\nChart: {}\n",
            self.symbol, self.systemSymbol, self.type_field, self.x, self.y, self.chart
        );

        if !self.orbitals.is_empty() {
            for orb in &self.orbitals {
                disp = disp.to_owned() + format!("{}", orb).as_str();
            }
        }
        if self.faction != faction_check {
            disp = disp.to_owned() + format!("Faction: {}\n", self.faction.symbol).as_str();
        }
        for tr in &self.traits {
            disp = disp.to_owned() + format!("{}", tr).as_str();
        }
        if self.chart != char_check {
            disp = disp.to_owned() + format!("{}", self.chart).as_str();
        }

        writeln!(f, "{}", disp)
    }
}
