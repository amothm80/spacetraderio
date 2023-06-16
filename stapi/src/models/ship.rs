use std::fmt;

use crate::models::shipcargo;
use crate::models::shipcrew;
use crate::models::shipengine;
use crate::models::shipframe;
use crate::models::shipfuel;
use crate::models::shipmodule;
use crate::models::shipmount;
use crate::models::shipnav;
use crate::models::shipreactor;
use crate::models::shipregistration;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
/**
 * A ship
 */
pub struct Ship {
    /**
     * The globally unique identifier of the ship in the following format: `[AGENT_SYMBOL]_[HEX_ID]`
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
     * The ship's crew service and maintain the ship's systems and equipment.
     */
    pub crew: shipcrew::ShipCrew,
    /**
     * The frame of the ship. The frame determines the i64, of modules and mounting points of the ship, as well as base fuel capacity. As the condition of the frame takes more wear, the ship will become more sluggish and less maneuverable.
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
    pub cargo: shipcargo::ShipCargo,
    /**
     * Details of the ship's fuel tanks including how much fuel was consumed during the last transit or action.
     */
    pub fuel: shipfuel::ShipFuel,
}

impl fmt::Display for Ship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut disp = format!(
            "Ship Symbol: {:?}\n\nRegistration Data:\n{}\n\nNavigation Information:\n{}\n\n",
            self.symbol, self.registration, self.nav
        );
        disp = disp.to_owned()
            + format!(
                "Crew Information:\n{}\n\nFrame Information:\n{}\n\nReactor Information:\n{}\n\nEngine Information:\n{}\n\n",
                self.crew, self.frame, self.reactor,self.engine
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
        disp = disp.to_owned() + format!("\nCargo Information:\n{}\n", self.cargo).as_str();
        disp = disp.to_owned() + format!("\nFuel:\n{}\n", self.fuel).as_str();
        write!(f, "{}", disp)
    }
}
