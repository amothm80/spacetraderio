use std::fmt;

use crate::models::surveydeposit;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum SurveySize {
    #[default]
    SMALL,
    MODERATE,
    LARGE,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
/**
 * A resource survey of a waypoint, detailing a specific extraction location and the types of resources that can be found there.
 */
pub struct Survey {
    /**
     * A unique signature for the location of this survey. This signature is verified when attempting an extraction using this survey.
     */
    pub signature: String,
    /**
     * The symbol of the waypoint that this survey is for.
     */
    pub symbol: String,
    /**
     * A list of deposits that can be found at this location.
     */
    pub deposits: Vec<surveydeposit::SurveyDeposit>,
    /**
     * The date and time when the survey expires. After this date and time, the survey will no longer be available for extraction.
     */
    pub expiration: String,
    /**
     * The size of the deposit. This value indicates how much can be extracted from the survey before it is exhausted.
     */
    pub size: SurveySize,
}

impl fmt::Display for Survey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut disp = format!(
            "Signature: {}\nSymbol: {}\nExpiration: {}\nSize: {:#?}\n",
            self.signature, self.symbol, self.expiration, self.size
        );

        if !self.deposits.is_empty() {
            for dep in &self.deposits {
                disp = disp.to_owned() + format!("{}", dep).as_str();
            }
        }

        write!(f, "{}", disp)
    }
}
