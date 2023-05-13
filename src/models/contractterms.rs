use crate::models::contractdelivergood;
use crate::models::contractpayment;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractTerms {
    /**
     * The deadline for the contract.
     */
    pub deadline: String,
    pub payment: contractpayment::ContractPayment,
    pub deliver: Vec<contractdelivergood::ContractDeliverGood>,
}
