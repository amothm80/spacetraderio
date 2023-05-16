use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractPayment {
    /**
     * The amount of credits received up front for accepting the contract.
     */
    pub onAccepted: i64,
    /**
     * The amount of credits received when the contract is fulfilled.
     */
    pub onFulfilled: i64,
}
