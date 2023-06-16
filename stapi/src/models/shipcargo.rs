use std::fmt;

use crate::models::shipcargoitem;
use crate::models::shipcargoitem::ShipCargoItem;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct ShipCargo {
    /**
     * The max i64, of items that can be stored in the cargo hold.
     */
    pub capacity: i64,
    /**
     * The i64, of items currently stored in the cargo hold.
     */
    pub units: i64,
    /**
     * The items currently in the cargo hold.
     */
    pub inventory: Vec<shipcargoitem::ShipCargoItem>,
}

impl fmt::Display for ShipCargo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inv_check = ShipCargoItem::default();
        let mut disp = format!(
            "Ship cargo bay is at {} units out of capacity of {} units\nCargo Inventory:\n",
            self.units, self.capacity
        );
        for item in &self.inventory {
            if *item != inv_check {
                disp = disp.to_owned() + format!("{}", item).as_str() + "\n";
            }
        }
        write!(f, "{}", disp)
    }
}
