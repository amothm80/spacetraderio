use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct Data{
    pub data: ContractDeliverGood,
}
/**
 * The details of a delivery contract. Includes the type of good, units needed, and the destination.
 */
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractDeliverGood {
    /**
     * The symbol of the trade good to deliver.
     */
    pub tradeSymbol: String,
    /**
     * The destination where goods need to be delivered.
     */
    pub destinationSymbol: String,
    /**
     * The number of units that need to be delivered on this contract.
     */
    pub unitsRequired: i64,
    /**
     * The number of units fulfilled on this contract.
     */
    pub unitsFulfilled: i64,
  }
  