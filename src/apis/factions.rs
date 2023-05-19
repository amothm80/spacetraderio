use crate::apis::config;
use crate::apis::errors;
use crate::models::faction;
use crate::models::message;
//use reqwest::Error;
//use reqwest::Response;
//use serde_derive::{Serialize,Deserialize};
//use serde_json::Map;
//use serde_json::Value;
//use serde::Deserialize;

pub async fn get_factions(
    config: &config::Config,
) -> Result<Vec<faction::Faction>, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::GET,
        config.base_path.to_owned() + "/factions",
    );
    reqbuilder = reqbuilder.bearer_auth(config.bearer_token.to_owned());
    let req = reqbuilder.build().unwrap();
    let resp = client.execute(req).await.unwrap();
    let status = resp.status();
    let json = resp.json::<message::MessageFactions>().await?;
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}

pub async fn get_faction(
    config: &config::Config,
    factionsymbol: String,
) -> Result<faction::Faction, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::GET,
        config.base_path.to_owned() + "/factions/".to_owned().as_str() + factionsymbol.as_str(),
    );
    reqbuilder = reqbuilder.bearer_auth(config.bearer_token.to_owned());
    let req = reqbuilder.build().unwrap();
    let resp = client.execute(req).await.unwrap();
    let status = resp.status();
    let json = resp.json::<message::MessageFaction>().await?;
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}
