use crate::models::contractterms;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fmt;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum ContractType {
    #[default]
    PROCUREMENT,
    TRANSPORT,
    SHUTTLE,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Contract {
    pub id: String,
    /**
     * The symbol of the faction that this contract is for.
     */
    pub factionSymbol: String,
    #[serde(rename = "type")]
    pub type_field: ContractType,
    pub terms: contractterms::ContractTerms,
    /**
     * Whether the contract has been accepted by the agent
     */
    pub accepted: bool,
    /**
     * Whether the contract has been fulfilled
     */
    pub fulfilled: bool,
    /**
     * The time at which the contract expires
     */
    pub expiration: String,
}

impl fmt::Display for Contract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.type_field {
            ContractType::PROCUREMENT => {
                let mut disp = format!(
                    "Contract number ({}): You are requested to deliver the items below:",
                    self.id
                );
                disp = disp.to_owned() + "\n";
                for term in self.terms.deliver.iter() {
                    disp = disp.to_owned() + format!("{} units of {} from destination {}, out of which, {} units have been fulfilled.", 
                        term.unitsRequired, term.tradeSymbol, term.destinationSymbol, term.unitsFulfilled).as_str();
                    disp = disp.to_owned() + "\n";
                }
                disp = disp.to_owned()
                    + format!(
                        "You will be paid {} credits on acceptance, and {} credits on fulfillment.",
                        self.terms.payment.onAccepted, self.terms.payment.onFulfilled
                    )
                    .as_str();
                disp = disp.to_owned() + "\n";
                if self.fulfilled {
                    disp = disp.to_owned() + format!("The contract has been fulfilled").as_str();
                } else {
                    disp = disp.to_owned()
                        + format!("The contract is set to expire on {}.", self.expiration).as_str();
                }
                write!(f, "{}", disp)
            }
            _ => todo!(),
        }
        //    for term in self.terms.deliver.iter(){
        //     writeln!(f,"{} units of {} from destination {}, out of which, {} has been fulfilled.",
        //         term.unitsRequired, term.tradeSymbol, term.destinationSymbol, term.unitsFulfilled);
        //    }
        //    writeln!(f,"The contract is set to expire on {}.", self.expiration);
        //    Ok(())
    }
}
