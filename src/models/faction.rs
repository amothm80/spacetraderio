use std::fmt;

use crate::models::factiontrait;
use serde_derive::Deserialize;
use serde_derive::Serialize;

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// //#[serde(rename_all = "camelCase")]
// #[allow(non_camel_case_types)]
// #[allow(non_snake_case)]
// pub struct Factions {
//     pub data: Vec<Faction>,
// }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Faction {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub headquarters: String,
    pub traits: Vec<factiontrait::FactionTrait>,
}

impl fmt::Display for Faction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut disp = format!("{} ({})", self.name, self.symbol);
        disp = disp.to_owned() + "\n";
        disp = disp.to_owned() + self.description.as_str() + "\n";
        disp = disp.to_owned() + format!("HQ: {}", self.headquarters).as_str() + "\n";
        disp = disp.to_owned() + "Traits: ";
        for tr in &self.traits {
            disp = disp.to_owned() + format!(" {} ", tr.name).as_str();
        }
        write!(f, "{}", disp)
    }
}
