use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct Data{
    pub data: ExtractionYield,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtractionYield {
    pub symbol: String,
    /**
     * The number of units extracted that were placed into the ship's cargo hold.
     */
    pub units: i64,
  }
  