use crate::models::factiontrait;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct Data {
    pub data: Faction,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Faction {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub headquarters: String,
    pub traits: Vec<factiontrait::FactionTrait>,
}
