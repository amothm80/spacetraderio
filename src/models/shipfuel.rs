use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Data {
    pub data: ShipFuel,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
/**
 * Details of the ship's fuel tanks including how much fuel was consumed during the last transit or action.
 */
pub struct ShipFuel {
    /**
     * The current amount of fuel in the ship's tanks.
     */
    pub current: i64,
    /**
     * The maximum amount of fuel the ship's tanks can hold.
     */
    pub capacity: i64,
    pub consumed: ShipFuelConsumed,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipFuelConsumed {
    /**
     * The amount of fuel consumed by the most recent transit or action.
     */
    pub amount: i64,
    /**
     * The time at which the fuel was consumed.
     */
    pub timestamp: i64,
}
