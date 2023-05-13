use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub data: SurveyDeposit,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/**
 * A surveyed deposit of a mineral or resource available for extraction.
 */
pub struct SurveyDeposit {
    /**
     * The symbol of the deposit.
     */
    pub symbol: String,
}
