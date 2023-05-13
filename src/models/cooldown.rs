use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct Data {
    pub data: Cooldown,
}

/**
 * A cooldown is a period of time in which a ship cannot perform certain actions.
 */
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cooldown {
    /**
     * The symbol of the ship that is on cooldown
     */
    pub shipSymbol: String,
    /**
     * The total duration of the cooldown in seconds
     */
    pub totalSeconds: i64,
    /**
     * The remaining duration of the cooldown in seconds
     */
    pub remainingSeconds: i64,
    /**
     * The date and time when the cooldown expires in ISO 8601 format
     */
    pub expiration: String,
}
