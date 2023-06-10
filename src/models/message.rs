use crate::models::*;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fmt;

use super::meta::Meta;
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
        write!(f, "{}\n", self.nav)
    }
}

///////////////////////////////////////////////
///Errors
//////////////////////////////////////////////

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Error {
    pub error: ErrorContent,
}
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
            "Error code: {}, Error message: {}",
            self.code, self.message
        )
    }
}
