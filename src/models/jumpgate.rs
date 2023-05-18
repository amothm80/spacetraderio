use crate::models::connectedsystem;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Data {
    pub data: JumpGate,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct JumpGate {
    /**
     * The maximum jump range of the gate.
     */
    pub jumpRange: i64,
    /**
     * The symbol of the faction that owns the gate.
     */
    pub factionSymbol: String,
    /**
     * The systems within range of the gate that have a corresponding gate.
     */
    pub connectedSystems: Vec<connectedsystem::ConntectedSystem>,
}
