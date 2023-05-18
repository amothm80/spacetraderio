use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
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
