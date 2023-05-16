use crate::models::*;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fmt::{self, write};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Message {
    data(Data),
    error(ErrorContent),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Data {
    chart(chart::Chart),
    agent(agent::Agent),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub error: ErrorContent,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorContent {
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub symbol: String,
    #[serde(default)]
    pub code: i32,
}

impl fmt::Display for ErrorContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "Error code: {}, Error message: {}",self.code,self.message)
    }
}
