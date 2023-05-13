use crate::models::shipnav;
use crate::models::shipregistration;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub data: ScannedShip,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScannedShipFrame {
    symbol: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScannedShipReactor {
    symbol: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScannedShipEngine {
    symbol: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScannedShipMount {
    symbol: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/**
 * The ship that was scanned. Details include information about the ship that could be detected by the scanner.
 */
pub struct ScannedShip {
    /**
     * The globally unique identifier of the ship.
     */
    pub symbol: String,
    /**
     * The public registration information of the ship
     */
    pub registration: shipregistration::ShipRegistration,
    /**
     * The navigation information of the ship.
     */
    pub nav: shipnav::ShipNav,
    /**
     * The frame of the ship.
     */
    pub frame: ScannedShipFrame,
    /**
     * The reactor of the ship.
     */
    pub reactor: ScannedShipReactor,
    /**
     * The engine of the ship.
     */
    pub engine: ScannedShipEngine,
    pub mounts: Vec<ScannedShipMount>,
}
