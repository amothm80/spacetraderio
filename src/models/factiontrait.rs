use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Data {
    pub data: FactionTrait,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[allow(non_camel_case_types)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum FactionTraitSymbol {
    #[default]
    BUREAUCRATIC,
    SECRETIVE,
    CAPITALISTIC,
    INDUSTRIOUS,
    PEACEFUL,
    DISTRUSTFUL,
    WELCOMING,
    SMUGGLERS,
    SCAVENGERS,
    REBELLIOUS,
    EXILES,
    PIRATES,
    RAIDERS,
    CLAN,
    GUILD,
    DOMINION,
    FRINGE,
    FORSAKEN,
    ISOLATED,
    LOCALIZED,
    ESTABLISHED,
    NOTABLE,
    DOMINANT,
    INESCAPABLE,
    INNOVATIVE,
    BOLD,
    VISIONARY,
    CURIOUS,
    DARING,
    EXPLORATORY,
    RESOURCEFUL,
    FLEXIBLE,
    COOPERATIVE,
    UNITED,
    STRATEGIC,
    INTELLIGENT,
    RESEARCH_FOCUSED,
    COLLABORATIVE,
    PROGRESSIVE,
    MILITARISTIC,
    TECHNOLOGICALLY_ADVANCED,
    AGGRESSIVE,
    IMPERIALISTIC,
    TREASURE_HUNTERS,
    DEXTEROUS,
    UNPREDICTABLE,
    BRUTAL,
    FLEETING,
    ADAPTABLE,
    SELF_SUFFICIENT,
    DEFENSIVE,
    PROUD,
    DIVERSE,
    INDEPENDENT,
    SELF_INTERESTED,
    FRAGMENTED,
    COMMERCIAL,
    FREE_MARKETS,
    ENTREPRENEURIAL,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct FactionTrait {
    /**
     * The unique identifier of the trait.
     */
    pub symbol: FactionTraitSymbol,

    /**
     * The name of the trait.
     */
    pub name: String,
    /**
     * A description of the trait.
     */
    pub description: String,
}
