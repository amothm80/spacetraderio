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
