pub mod apis;
pub mod models;

use apis::agent;
use apis::config::Config;
use apis::errors;
use apis::factions;
use apis::ships;
use apis::systems;
use std::env;
use std::fs;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();
    let token = fs::read_to_string("token.txt")?;
    let mut conf = Config::new();
    conf.bearer_token = token;
    //let agent = agent::get_my_agent(&conf).await;
    //let mut contracts: Vec<models::contract::Contract> = vec![];
    match args[1].as_str() {
        "agent" => println!("{}", agent::get_my_agent(&conf).await?),
        "faction" => match args[2].as_str() {
            "all" => {
                let factions = factions::get_factions(&conf).await?;
                for faction in factions {
                    println!("{}\n", faction);
                }
            }
            _ => println!(
                "{}",
                factions::get_faction(&conf, args[2].to_owned()).await?
            ),
        },
        "contract" => match args[2].as_str() {
            "all" => {
                let contracts = agent::get_my_contracts(&conf).await?;
                for contract in contracts {
                    println!("{}\n", contract);
                }
            }
            "accept" => {
                println!(
                    "{}\nACCEPTED",
                    agent::accept_contract(&conf, args[3].to_owned()).await?
                )
            }
            // match agent::accept_contract(&conf, args[3].to_owned()).await {
            //     Ok(s) => println!("{}\nACCEPTED", s),
            //     Err(s) => handle_errors(s),
            // },
            "fulfill" => {
                println!(
                    "{}\nFULILLED",
                    agent::fulfill_contract(&conf, args[3].to_owned()).await?
                );
            }
            _ => {
                println!("{}", agent::get_contract(&conf, args[2].to_owned()).await?)
            }
        },
        "ships" => {
            let ships = ships::get_my_ships(&conf).await?;
            for ship in ships {
                println!("{}\n\n", ship);
            }
        }
        _ => println!("invalid argument"),
    }
    //println!("{:#?}",systems::get_systems(&conf).await);
    //println!("{:#?}",systems::get_system_waypoints(&conf, String::from("X1-ZA40")).await);
    //println!("{:#?}",systems::get_system_waypoint(&conf, String::from("X1-ZA40"), String::from("X1-ZA40-41138D")).await);
    //println!("{:#?}",systems::get_waypoint_market(&conf, String::from("X1-ZA40"), String::from("X1-ZA40-41138D")).await);
    //println!("{:#?}",systems::get_waypoint_shipyard(&conf, String::from("X1-ZA40"), String::from("X1-ZA40-41138D")).await);
    //println!("{:#?}",systems::get_waypoint_jumpgate(&conf, String::from("X1-ZA40"), String::from("X1-ZA40-41138D")).await);

    Ok(())
}

fn handle_errors(e: errors::STError) {
    match e {
        errors::STError::reqwesterror(e) => println!("{}", e),
        errors::STError::serdejerror(e) => println!("{}", e),
        errors::STError::stapierror(e) => println!("{}", e),
        errors::STError::stgeneralerror => println!("{}", e),
    }
}

// let mut conf = Configuration::new();
// conf.bearer_access_token = Some(token);
// let myagent = agents_api::get_my_agent(&conf);
// println!("{:#?}",myagent.await?);

//let client = reqwest::Client::new();
//let mut headers = HeaderMap::new();
////let resp = client.get(ST_API_URL).send().await?.json::<HashMap<String, Value>>().await?;
////let resp = client.get(ST_API_URL.to_owned()+ST_API_AGENT).bearer_auth(token).send().await?;//.json::<HashMap<String, Value>>().await?;
//let resp = client
//    .get("https://api.spacetraders.io/v2/my/contracts/clhi30h5t10hfs60dompcbn9p")
//    .bearer_auth(token)
//    .send()
//    .await?;
////let json = resp.json::<RootResp>().await?;
//println!("{:#?}", resp.text().await?);
////println!("{:#?}", resp.json::<models::contract::Contract>().await?);
////let agent = resp.json::<models::agent::Data>().await?.data;
////println!("{:#?}", agent);

// let mut map = HashMap::new();
// map.insert("lang", "rust");
// map.insert("body", "json");

// let client = reqwest::Client::new();
// let res = client.post("http://httpbin.org/post")
//     .json(&map)
//     .send()
//     .await?;
