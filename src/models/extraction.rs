use serde_derive::Deserialize;
use serde_derive::Serialize;
use crate::models::extractionyield;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data{
    pub data: Extraction,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extraction {
    pub shipSymbol: String,
    #[serde(rename = "yield")]
    pub yield_field: extractionyield::ExtractionYield,
  }

