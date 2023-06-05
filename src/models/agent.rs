use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Agent {
    pub accountId: String,
    pub symbol: String,
    /**
     * The headquarters of the agent.
     */
    pub headquarters: String,
    /**
     * The number of credits the agent has available. Credits can be negative if funds have been overdrawn.
     */
    pub credits: i64,
}

impl fmt::Display for Agent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let disp = format!(
            "Account ID: {}\nSymbol    : {}\nHQ        : {}\nCredits   : {}",
            self.accountId, self.symbol, self.headquarters, self.credits
        );
        write!(f, "{}", disp)
    }
}
