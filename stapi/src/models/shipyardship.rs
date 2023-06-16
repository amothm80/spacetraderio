use std::fmt;

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

impl fmt::Display for ShipyardShip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut disp = format!(
            "Ship Symbol: {}\nShip Type: {:?}\nShip Name: {}\nShip Description: {}\nPurchase Price: {}\n",
            self.symbol, self.type_field, self.name, self.description, self.purchasePrice
        );
        disp = disp.to_owned()
            + format!(
                "Frame Information:\n{}\n\nReactor Information:\n{}\n\nEngine Information:\n{}\n\n",
                self.frame, self.reactor, self.engine
            )
            .as_str();
        disp = disp.to_owned() + "Module Information:\n";
        for module in &self.modules {
            disp = disp.to_owned() + format!("{}", module).as_str()
        }
        disp = disp.to_owned() + "\nMount Information:\n";
        for mount in &self.mounts {
            disp = disp.to_owned() + format!("{}", mount).as_str()
        }
        write!(f, "{}", disp)
    }
}
