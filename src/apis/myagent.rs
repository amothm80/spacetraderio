use crate::apis::config;
use crate::apis::errors;
use crate::models::agent;
use crate::models::contract;
use crate::models::message;
use reqwest::Error;

pub async fn get_my_agent(config: &config::Config) -> Result<agent::Agent, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::GET,
        config.base_path.to_owned() + "/my/agent",
    );
    reqbuilder = reqbuilder.bearer_auth(config.bearer_token.to_owned());
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let json = resp.json::<message::Message>().await?;
    if !status.is_client_error() && !status.is_server_error() {
        if let message::Data::agent(a) = json.data {
            Ok(a)
        }else{
            Err(errors::STError::stgeneralerror)
        } 
    }else{
        Err(errors::STError::stapierror(json.error))
    }
}

pub async fn get_my_contracts(config: &config::Config) -> Result<Vec<contract::Contract>, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::GET,
        config.base_path.to_owned() + "/my/contracts",
    );
    reqbuilder = reqbuilder.bearer_auth(config.bearer_token.to_owned());
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let s = resp.text().await.unwrap();
    //let s2 = s.clone();
    println!("{:#?}",s);
    

    let j = r#"
    {"data":[{"id":"clhmdeetx02v5s60db5upt1mt","factionSymbol":"COSMIC","type":"PROCUREMENT","terms":{"deadline":"2023-05-20T19:18:29.012Z","payment":{"onAccepted":121440,"onFulfilled":485760},"deliver":[{"tradeSymbol":"IRON_ORE","destinationSymbol":"X1-ZA40-15970B","unitsRequired":13200,"unitsFulfilled":0}]},"accepted":false,"fulfilled":false,"expiration":"2023-05-16T19:18:29.012Z"}],"meta":{"total":1,"page":1,"limit":10}}
   "#;

//    "meta":{
//     "total":7,
//     "page":1,
//     "limit":10
// }

    let deserializer = &mut serde_json::Deserializer::from_str(j);
    let result: std::result::Result<message::Message, _> = serde_path_to_error::deserialize(deserializer);
    match result {
        Ok(_) => println!("Expected an error"),
        Err(err) => {
            panic!("{},{}", err, err.path());
        }
    }        


    let json = serde_json::from_str::<message::Message>(&s).unwrap();
    //let json = resp.json::<message::Message>().await?;
    if !status.is_client_error() && !status.is_server_error() {
        if let message::Data::contracts(a) = json.data {
            Ok(a)
        }else{
            Err(errors::STError::stgeneralerror)
        } 
    }else{
        Err(errors::STError::stapierror(json.error))
    }


}

//println!("status: {:#?} , {:#?}",resp.status(), resp.text().await?);
//println!("status: {:#?} , {:#?}",resp.status(), resp.json::<message::Message>().await?);
//let status = resp.status();
// if !status.is_client_error() && !status.is_server_error() {
//     Ok(resp.json::<agent::Data>().await?.data)
// }else{

// }
// match resp.error_for_status_ref(){
//     Ok(_res) => {Ok(resp.json::<agent::Data>().await?.data)},
//     Err(err) => {Err(errors::STError::reqwesterror(err))},
// }
//Ok(agent::Agent { accountId: String::from(""), symbol: String::from(""), headquarters: String::from(""), credits: 0 })

