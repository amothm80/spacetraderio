use serde_derive::Deserialize;
use serde_derive::Serialize;
use crate::models::contractterms;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data{
    pub data: Contract,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ContractType{
    #[default]
    PROCUREMENT,
    TRANSPORT,
    SHUTTLE,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub id: String,
    /**
     * The symbol of the faction that this contract is for.
     */
    pub factionSymbol: String,
    #[serde(rename = "type")]
    pub type_field: ContractType,
    pub terms: Vec<contractterms::ContractTerms>,
    /**
     * Whether the contract has been accepted by the agent
     */
    pub accepted: bool,
    /**
     * Whether the contract has been fulfilled
     */
    pub fulfilled: bool,
    /**
     * The time at which the contract expires
     */
    pub expiration: String,
  }
  