use crate::models::*;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fmt;

use super::cooldown::Cooldown;
use super::markettransaction::MarketTransaction;
use super::meta::Meta;
use super::shipcargo::ShipCargo;
use super::shipfuel::ShipFuel;
use super::shipnav::ShipNav;
// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// //#[allow(non_camel_case_types)]
// #[allow(non_camel_case_types)]
// #[allow(non_snake_case)]
// pub struct Message {
//     #[serde(default)]
//     pub data: Data,
//     #[serde(default)]
//     pub meta: Meta,
//     #[serde(default)]
//     pub error: ErrorContent,
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageMyAgent {
    #[serde(default)]
    pub data: agent::Agent,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

//REGISTER AGENT
/////////////////
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageAgentRegister {
    #[serde(default)]
    pub data: MessageAgentRegisterData,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageAgentRegisterData {
    #[serde(default)]
    pub agent: agent::Agent,
    #[serde(default)]
    pub contract: contract::Contract,
    #[serde(default)]
    pub faction: faction::Faction,
    #[serde(default)]
    pub ship: ship::Ship,
    #[serde(default)]
    pub token: String,
}

impl fmt::Display for MessageAgentRegisterData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Registration Details:\nAgent:\n{}\n\nContract:\n{}\n\nFaction:\n{}\n\nShip:\n{}\n\nToken:\n{}\n",
    self.agent, self.contract, self.faction, self.ship, self.token)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageMyContracts {
    #[serde(default)]
    pub data: Vec<contract::Contract>,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageContract {
    #[serde(default)]
    pub data: contract::Contract,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageContractAcceptance {
    #[serde(default)]
    pub data: MessageContractAcceptanceData,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageContractAcceptanceData {
    #[serde(default)]
    pub contract: contract::Contract,
    #[serde(default)]
    pub agent: agent::Agent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageContractFulfillment {
    #[serde(default)]
    pub data: MessageContractFulfillmentData,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageContractFulfillmentData {
    #[serde(default)]
    pub agent: agent::Agent,
    #[serde(default)]
    pub contract: contract::Contract,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageFaction {
    #[serde(default)]
    pub data: faction::Faction,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageFactions {
    #[serde(default)]
    pub data: Vec<faction::Faction>,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageMyShips {
    #[serde(default)]
    pub data: Vec<ship::Ship>,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageSystem {
    #[serde(default)]
    pub data: system::System,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageSystems {
    #[serde(default)]
    pub data: Vec<system::System>,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageWaypoint {
    #[serde(default)]
    pub data: waypoint::Waypoint,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageWaypoints {
    #[serde(default)]
    pub data: Vec<waypoint::Waypoint>,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageMarket {
    #[serde(default)]
    pub data: market::Market,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipyard {
    #[serde(default)]
    pub data: shipyard::Shipyard,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageJumpgate {
    #[serde(default)]
    pub data: jumpgate::JumpGate,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

/////////////////////////////////////////////////////////////
//SHIP MESSAGES
////////////////////////////////////////////////////////////

//SHIP PURCHASE
///////////////

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipPurchase {
    #[serde(default)]
    pub data: MessageShipPurchaseData,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipPurchaseData {
    #[serde(default)]
    pub agent: agent::Agent,
    #[serde(default)]
    pub ship: ship::Ship,
    #[serde(default)]
    pub transaction: shipyardtransaction::ShipyardTransaction,
}

impl fmt::Display for MessageShipPurchaseData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}\n{}", self.agent, self.ship, self.transaction)
    }
}

//SHIP NAVIGATION
/////////////////
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipNavigation {
    #[serde(default)]
    pub data: MessageShipNavigationData,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipNavigationData {
    #[serde(default)]
    pub fuel: shipfuel::ShipFuel,
    #[serde(default)]
    pub shipnav: shipnav::ShipNav,
    #[serde(default)]
    pub transaction: shipyardtransaction::ShipyardTransaction,
}

impl fmt::Display for MessageShipNavigationData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}\n{}", self.fuel, self.shipnav, self.transaction)
    }
}

//SHIP CARGO
/////////////////
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipCargo {
    #[serde(default)]
    pub data: shipcargo::ShipCargo,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

//SHIP NAVIGATION
/////////////////
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipOrbit {
    #[serde(default)]
    pub data: MessageShipOrbitData,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipOrbitData {
    #[serde(default)]
    pub nav: shipnav::ShipNav,
}

impl fmt::Display for MessageShipOrbitData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.nav)
    }
}

//SHIP CHART
/////////////////
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipChart {
    #[serde(default)]
    pub data: MessageShipChartData,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipChartData {
    #[serde(default)]
    pub chart: chart::Chart,
    #[serde(default)]
    pub waypoint: waypoint::Waypoint,
}

impl fmt::Display for MessageShipChartData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}\n", self.chart, self.waypoint)
    }
}

//SHIP COOLDOWN
/////////////////
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipCooldown {
    #[serde(default)]
    pub data: cooldown::Cooldown,
    #[serde(default)]
    pub no_cooldown: String,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

impl fmt::Display for MessageShipCooldown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}\n", self.data, self.no_cooldown)
    }
}

//SHIP CHART
/////////////////
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipSurvey {
    #[serde(default)]
    pub data: MessageShipSurveyData,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipSurveyData {
    #[serde(default)]
    pub cooldown: chart::Chart,
    #[serde(default)]
    pub surveys: MessageShipSurveyDataSurveys,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipSurveyDataSurveys {
    #[serde(default)]
    pub items: Vec<survey::Survey>,
}

impl fmt::Display for MessageShipSurveyData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut disp = format!("{}", self.cooldown);
        if !self.surveys.items.is_empty() {
            for item in &self.surveys.items {
                disp = disp.to_owned() + format!("{}\n", item).as_str();
            }
        }

        writeln!(f, "{}", disp)
    }
}

//SHIP EXTRACT RESOURCE
/////////////////
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipExtract {
    #[serde(default)]
    pub data: MessageShipExtractData,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipExtractData {
    #[serde(default)]
    pub cooldown: cooldown::Cooldown,
    #[serde(default)]
    pub extraction: extraction::Extraction,
    #[serde(default)]
    pub cargo: ShipCargo,
}

impl fmt::Display for MessageShipExtractData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}\n{}\n{}", self.cooldown, self.extraction, self.cargo)
    }
}

//SHIP JETTISON CARGO
/////////////////
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipJettison {
    #[serde(default)]
    pub data: MessageShipJettisonData,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipJettisonData {
    #[serde(default)]
    pub cargo: ShipCargo,
}

impl fmt::Display for MessageShipJettisonData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.cargo)
    }
}

//SHIP JUMP
/////////////////
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipJump {
    #[serde(default)]
    pub data: MessageShipJumpData,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipJumpData {
    #[serde(default)]
    pub cooldown: Cooldown,
    #[serde(default)]
    pub nav: ShipNav,
}

impl fmt::Display for MessageShipJumpData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}\n{}", self.cooldown, self.nav)
    }
}

//SHIP NAV
/////////////////
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipNav {
    #[serde(default)]
    pub data: shipnav::ShipNav,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

impl fmt::Display for MessageShipNav {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.data)
    }
}

//SHIP WARP
/////////////////
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipWarp {
    #[serde(default)]
    pub data: MessageShipWarpData,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipWarpData {
    #[serde(default)]
    pub fuel: shipfuel::ShipFuel,
    #[serde(default)]
    pub nav: ShipNav,
}

impl fmt::Display for MessageShipWarpData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}\n{}", self.fuel, self.nav)
    }
}

//SHIP SELL CARGO
/////////////////
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipSellCargo {
    #[serde(default)]
    pub data: MessageShipSellCargoData,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipSellCargoData {
    #[serde(default)]
    pub agent: agent::Agent,
    #[serde(default)]
    pub cargo: shipcargo::ShipCargo,
    #[serde(default)]
    pub transaction: markettransaction::MarketTransaction,
}

impl fmt::Display for MessageShipSellCargoData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}\n{}\n{}", self.agent, self.cargo, self.transaction)
    }
}

//SHIP SCAN SYSTEMS
/////////////////
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipScanSystems {
    #[serde(default)]
    pub data: MessageShipScanSystemsData,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipScanSystemsData {
    #[serde(default)]
    pub cooldown: cooldown::Cooldown,
    #[serde(default)]
    pub systems: MessageShipScanSystemsDataScannedSystems,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipScanSystemsDataScannedSystems {
    #[serde(default)]
    pub items: Vec<scannedsystem::ScannedSystem>,
}

impl fmt::Display for MessageShipScanSystemsData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut disp = format!("{}\n", self.cooldown);
        if !self.systems.items.is_empty() {
            for item in &self.systems.items {
                disp = disp.to_owned() + format!("System Data:\n{}", item).as_str();
            }
        }

        writeln!(f, "{}", disp)
    }
}

//SHIP SCAN WAYPOINTS
/////////////////
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipScanWaypoints {
    #[serde(default)]
    pub data: MessageShipScanWaypointsData,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipScanWaypointsData {
    #[serde(default)]
    pub cooldown: cooldown::Cooldown,
    #[serde(default)]
    pub waypoints: MessageShipScanWaypointsDataScannedWaypoints,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipScanWaypointsDataScannedWaypoints {
    #[serde(default)]
    pub items: Vec<scannedwaypoint::ScannedWaypoint>,
}

impl fmt::Display for MessageShipScanWaypointsData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut disp = format!("{}\n", self.cooldown);
        if !self.waypoints.items.is_empty() {
            for item in &self.waypoints.items {
                disp = disp.to_owned() + format!("Waypoint Data:\n{}", item).as_str();
            }
        }

        writeln!(f, "{}", disp)
    }
}

//SHIP SCAN SHIPS
/////////////////
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipScanShips {
    #[serde(default)]
    pub data: MessageShipScanShipsData,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipScanShipsData {
    #[serde(default)]
    pub cooldown: cooldown::Cooldown,
    #[serde(default)]
    pub waypoints: MessageShipScanShipsDataScannedShips,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipScanShipsDataScannedShips {
    #[serde(default)]
    pub items: Vec<scannedship::ScannedShip>,
}

impl fmt::Display for MessageShipScanShipsData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut disp = format!("{}\n", self.cooldown);
        if !self.waypoints.items.is_empty() {
            for item in &self.waypoints.items {
                disp = disp.to_owned() + format!("Waypoint Data:\n{}", item).as_str();
            }
        }

        writeln!(f, "{}", disp)
    }
}

//SHIP REFUEL SHIPS
/////////////////
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipRefuel {
    #[serde(default)]
    pub data: MessageShipRefuelData,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipRefuelData {
    #[serde(default)]
    pub agent: agent::Agent,
    #[serde(default)]
    pub fuel: ShipFuel,
}

impl fmt::Display for MessageShipRefuelData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}\n{}", self.agent, self.fuel)
    }
}

//SHIP PURCHASE CARGO
/////////////////
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipPurchaseCargo {
    #[serde(default)]
    pub data: MessageShipPurchaseCargoData,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipPurchaseCargoData {
    #[serde(default)]
    pub agent: agent::Agent,
    #[serde(default)]
    pub cargo: ShipCargo,
    #[serde(default)]
    pub transaction: MarketTransaction,
}

impl fmt::Display for MessageShipPurchaseCargoData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}\n{}\n{}\n", self.agent, self.cargo, self.transaction)
    }
}

//SHIP TRANSFER CARGO
/////////////////
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipTransferCargo {
    #[serde(default)]
    pub data: MessageShipTransferCargoData,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub error: ErrorContent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MessageShipTransferCargoData {
    #[serde(default)]
    pub cargo: ShipCargo,
}

impl fmt::Display for MessageShipTransferCargoData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}\n", self.cargo)
    }
}

///////////////////////////////////////////////
///Errors
//////////////////////////////////////////////

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// //#[serde(rename_all = "camelCase")]
// #[allow(non_camel_case_types)]
// #[allow(non_snake_case)]
// pub struct Error {
//     pub error: ErrorContent,
// }
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct ErrorContent {
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub symbol: String,
    #[serde(default)]
    pub code: i32,
}

impl fmt::Display for ErrorContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error Code: {}, Error Message: {}",
            self.code, self.message
        )
    }
}
