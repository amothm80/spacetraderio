use serde_derive::Deserialize;
use serde_derive::Serialize;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct ExtractionYield {
    pub symbol: String,
    /**
     * The number of units extracted that were placed into the ship's cargo hold.
     */
    pub units: i64,
}
