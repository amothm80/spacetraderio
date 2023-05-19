pub mod apis;
pub mod models;

use apis::config::Config;
use apis::errors;
use apis::agent;
use apis::ships;
use apis::factions;
use apis::systems;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let token = fs::read_to_string("token.txt")?;
    let mut conf = Config::new();
    conf.bearer_token = String::from(token);
    //println!("{:#?}",agent::get_my_agent(&conf).await);
    //println!("{:#?}",factions::get_factions(&conf).await);
    //println!("{:#?}",factions::get_faction(&conf,String::from("CORSAIRS")).await);
    //println!("{:#?}",agent::get_my_contracts(&conf).await);
    //println!("{}", agent::get_contract(&conf, String::from("clhmdeetx02v5s60db5upt1mt")).await.unwrap() );
    //println!("{:#?}",agent::accept_contract(&conf, String::from("clhmdeetx02v5s60db5upt1mt")).await);
    //println!("{:#?}",agent::fulfill_contract(&conf, String::from("clhmdeetx02v5s60db5upt1mt")).await);
    //println!("{:#?}",ships::get_my_ships(&conf).await);
    //println!("{:#?}",systems::get_systems(&conf).await);
    //println!("{:#?}",systems::get_system_waypoints(&conf, String::from("X1-ZA40")).await);
    println!("{:#?}",systems::get_system_waypoint(&conf, String::from("X1-JJ48"), String::from("X1-JJ48-87750D")).await);
    
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
