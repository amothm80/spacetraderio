use std::fmt;

use crate::models::shipnav;
use crate::models::shipregistration;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct ScannedShipFrame {
    symbol: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct ScannedShipReactor {
    symbol: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct ScannedShipEngine {
    symbol: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct ScannedShipMount {
    symbol: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
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
    #[serde(default)]
    pub frame: ScannedShipFrame,
    /**
     * The reactor of the ship.
     */
    #[serde(default)]
    pub reactor: ScannedShipReactor,
    /**
     * The engine of the ship.
     */
    pub engine: ScannedShipEngine,
    #[serde(default)]
    pub mounts: Vec<ScannedShipMount>,
}

impl fmt::Display for ScannedShip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut disp = format!(
            "Symbol: {}\nFrame: {}\nReactor: {:#?}\nEngine: {}\n",
            self.symbol, self.frame.symbol, self.reactor.symbol, self.engine.symbol
        );

        disp = disp.to_owned() + format!("{}\n", self.registration).as_str();
        disp = disp.to_owned() + format!("{}\n", self.nav).as_str();
        disp = disp.to_owned() + "Mounts: ";
        if !self.mounts.is_empty() {
            for mount in &self.mounts {
                disp = disp.to_owned() + format!("{}\\", mount.symbol).as_str();
            }
        }

        writeln!(f, "{}\n", disp)
    }
}
