use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub data: Agent,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
