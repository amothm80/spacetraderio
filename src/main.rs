#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate url;
extern crate reqwest;

pub mod apis;
pub mod models;

use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

//use lib::apis::

use apis::configuration::Configuration;
use apis::agents_api;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, HOST};
const ST_API_URL: &str = "https://api.spacetraders.io/v2";
const ST_API_AGENT: &str = "/my/agent";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let token = fs::read_to_string("token.txt")?;
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
    Ok(())
}

// let mut map = HashMap::new();
// map.insert("lang", "rust");
// map.insert("body", "json");

// let client = reqwest::Client::new();
// let res = client.post("http://httpbin.org/post")
//     .json(&map)
//     .send()
//     .await?;
