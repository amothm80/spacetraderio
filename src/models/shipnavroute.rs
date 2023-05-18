use crate::models::shipnavroutewaypoint;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Data {
    pub data: ShipNavRoute,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
/**
 * The routing information for the ship's most recent transit or current location.
 */
pub struct ShipNavRoute {
    /**
     * The destination or departure of a ships nav route.
     */
    pub destination: shipnavroutewaypoint::ShipNavRouteWaypoint,
    /**
     * The destination or departure of a ships nav route.
     */
    pub departure: shipnavroutewaypoint::ShipNavRouteWaypoint,
    /**
     * The date time of the ship's departure.
     */
    pub departureTime: String,
    /**
     * The date time of the ship's arrival. If the ship is in-transit, this is the expected time of arrival.
     */
    pub arrival: String,
}
