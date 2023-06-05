use std::fmt;

use crate::models::shipnavflightmode;
use crate::models::shipnavroute;
use crate::models::shipnavstatus;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
/**
 * The navigation information of the ship.
 */
pub struct ShipNav {
    /**
     * The system symbol of the ship's current location.
     */
    pub systemSymbol: String,
    /**
     * The waypoint symbol of the ship's current location, or if the ship is in-transit, the waypoint symbol of the ship's destination.
     */
    pub waypointSymbol: String,
    /**
     * The routing information for the ship's most recent transit or current location.
     */
    pub route: shipnavroute::ShipNavRoute,
    /**
     * The current status of the ship
     */
    pub status: shipnavstatus::ShipNavStatus,
    /**
     * The ship's set speed when traveling between waypoints or systems.
     */
    pub flightMode: shipnavflightmode::ShipNavFlightMode,
}

impl fmt::Display for ShipNav {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Current System: {}\nCurrent Waypoint: {}\nCurrent Route: {}\nCurrent Status: {:?}\nFlight Mode: {:?}\n",
            self.systemSymbol, self.waypointSymbol, self.route, self.status, self.flightMode
        )
    }
}
