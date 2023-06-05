use std::fmt;

use crate::models::extractionyield;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Extraction {
    pub shipSymbol: String,
    #[serde(rename = "yield")]
    pub yield_field: extractionyield::ExtractionYield,
}

impl fmt::Display for Extraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Extraction by ship {} has {}",
            self.shipSymbol, self.yield_field
        )
    }
}
