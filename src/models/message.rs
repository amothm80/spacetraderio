use crate::models::*;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fmt;

use super::meta::Meta;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[allow(non_camel_case_types)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Message {
    #[serde(default)]
    pub data:Data,
    #[serde(default)]
    pub meta:Meta,
    #[serde(default)]
    pub error:ErrorContent,
}

#[derive(Default,Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
//#[allow(non_camel_case_types)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum Data {
    #[default]
    default,
    chart(chart::Chart),
    agent(agent::Agent),
    connectedsystem(connectedsystem::ConntectedSystem),
    contract(contract::Contract),
    contracts(Vec<contract::Contract>),
    contractdelivergood(contractdelivergood::ContractDeliverGood),
    contractpayment(contractpayment::ContractPayment),
    contractterms(contractterms::ContractTerms),
    cooldown(cooldown::Cooldown),
    extraction(extraction::Extraction),
    extractionyield(extractionyield::ExtractionYield),
    faction(faction::Faction),
    factions(Vec<faction::Faction>),
    factiontrait(factiontrait::FactionTrait),
    jumpgate(jumpgate::JumpGate),
    market(market::Market),
    markettradegood(markettradegood::MarketTradeGood),
    markettransaction(markettransaction::MarketTransaction),
    meta(meta::Meta),
    scannedship(scannedship::ScannedShip),
    scannedsystem(scannedsystem::ScannedSystem),
    scannedwaypoint(scannedwaypoint::ScannedWaypoint),
    ship(ship::Ship),
    shipcargo(shipcargo::ShipCargo),
    shipcargoitem(shipcargoitem::ShipCargoItem),
    shipcondition(shipcondition::ShipCondition),
    shipcrew(shipcrew::ShipCrew),
    shipengine(shipengine::ShipEngine),
    shipframe(shipframe::ShipFrame),
    shipfuel(shipfuel::ShipFuel),
    shipmodule(shipmodule::ShipModule),
    shipmount(shipmount::ShipMount),
    shipnav(shipnav::ShipNav),
    shipnavflightmode(shipnavflightmode::ShipNavFlightMode),
    shipnavroute(shipnavroute::ShipNavRoute),
    shipnavroutewaypoint(shipnavroutewaypoint::ShipNavRouteWaypoint),
    shipnavstatus(shipnavstatus::ShipNavStatus),
    shipreactor(shipreactor::ShipReactor),
    shipregistration(shipregistration::ShipRegistration),
    shiprequirements(shiprequirements::ShipRequirements),
    shiprole(shiprole::ShipRole),
    shiptype(shiptype::ShipType),
    shipyard(shipyard::Shipyard),
    shipyardship(shipyardship::ShipyardShip),
    shipyardtransaction(shipyardtransaction::ShipyardTransaction),
    survey(survey::Survey),
    surveydeposit(surveydeposit::SurveyDeposit),
    system(system::System),
    systemfaction(systemfaction::SystemFaction),
    systemtype(systemtype::SystemType),
    systemwaypoint(systemwaypoint::SystemWaypoint),
    tradegood(tradegood::TradeGood),
    tradesymbol(tradesymbol::TradeSymbol),
    waypoint(waypoint::Waypoint),
    waypointfaction(waypointfaction::WaypointFaction),
    waypointorbital(waypointorbital::WaypointOrbital),
    waypointtrait(waypointtrait::WaypointTrait),
    waypointtype(waypointtype::WaypointType),
}

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
       write!(f, "Error code: {}, Error message: {}",self.code,self.message)
    }
}
