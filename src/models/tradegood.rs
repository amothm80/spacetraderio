use std::fmt;

use crate::models::tradesymbol;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct TradeGood {
    pub symbol: tradesymbol::TradeSymbol,
    pub name: String,
    pub description: String,
}

impl fmt::Display for TradeGood {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let disp = format!("{} ({:?})\n", self.name, self.symbol);

        write!(f, "{}", disp)
    }
}
