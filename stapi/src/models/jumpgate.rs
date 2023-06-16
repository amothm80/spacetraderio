use std::fmt;

use crate::models::connectedsystem;
use serde_derive::Deserialize;
use serde_derive::Serialize;

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
    #[serde(default)]
    pub factionSymbol: String,
    /**
     * The systems within range of the gate that have a corresponding gate.
     */
    pub connectedSystems: Vec<connectedsystem::ConntectedSystem>,
}

impl fmt::Display for JumpGate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut disp = format!(
            "Jump Gate Information:\nJump Range: {}\nFaction Symbol: {}\nConnected Systems:\n",
            self.jumpRange, self.factionSymbol
        );

        for system in &self.connectedSystems {
            disp = disp.to_owned() + format!("{}\n", system).as_str();
        }

        write!(f, "{}", disp)
    }
}
