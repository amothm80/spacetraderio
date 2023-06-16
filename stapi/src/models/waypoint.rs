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
pub struct Waypoint {
    symbol: String,
    /**
     * The type of waypoint.
     */
    #[serde(rename = "type")]
    type_field: waypointtype::WaypointType,
    systemSymbol: String,
    x: i64,
    y: i64,
    orbitals: Vec<waypointorbital::WaypointOrbital>,
    #[serde(default)]
    faction: waypointfaction::WaypointFaction,
    /**
     * The traits of the waypoint.
     */
    traits: Vec<waypointtrait::WaypointTrait>,
    /**
     * The chart of a system or waypoint, which makes the location visible to other agents.
     */
    #[serde(default)]
    chart: chart::Chart,
}

impl fmt::Display for Waypoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let faction_check = waypointfaction::WaypointFaction::default();
        let char_check = chart::Chart::default();
        let mut disp = format!(
            "Symbol: {}\nType: {:?}\nSystem Symbol: {}\nCoordinates: {},{}\n",
            self.symbol, self.type_field, self.systemSymbol, self.x, self.y
        );

        for orb in &self.orbitals {
            disp = disp.to_owned() + format!("{}", orb).as_str();
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
