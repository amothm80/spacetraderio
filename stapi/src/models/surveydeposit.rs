use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
/**
 * A surveyed deposit of a mineral or resource available for extraction.
 */
pub struct SurveyDeposit {
    /**
     * The symbol of the deposit.
     */
    pub symbol: String,
}

impl fmt::Display for SurveyDeposit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.symbol)
    }
}
