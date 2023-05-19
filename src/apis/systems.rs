use crate::apis::config;
use crate::apis::errors;
use crate::models::system;
use crate::models::waypoint;
use crate::models::message;
//use reqwest::Error;

pub async fn get_systems(config: &config::Config) -> Result<Vec<system::System>, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::GET,
        config.base_path.to_owned() + "/systems",
    );
    reqbuilder = reqbuilder.bearer_auth(config.bearer_token.to_owned());
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let text = resp.text().await?;
    let json = serde_json::from_str::<message::MessageSystems>(&text).unwrap();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}

pub async fn get_system_waypoints(config: &config::Config,system:String) -> Result<Vec<waypoint::Waypoint>, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::GET,
        config.base_path.to_owned() + "/systems/".to_owned().as_str() + system.as_str()+"/waypoints".to_owned().as_str(),
    );
    reqbuilder = reqbuilder.bearer_auth(config.bearer_token.to_owned());
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let text = resp.text().await?;
    let json = serde_json::from_str::<message::MessageWaypoints>(&text).unwrap();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}

pub async fn get_system_waypoint(config: &config::Config,system:String, waypoint: String) -> Result<waypoint::Waypoint, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::GET,
        config.base_path.to_owned() + "/systems/".to_owned().as_str() + system.as_str()+"/waypoints/".to_owned().as_str() + waypoint.as_str(),
    );
    reqbuilder = reqbuilder.bearer_auth(config.bearer_token.to_owned());
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let text = resp.text().await?;
    let json = serde_json::from_str::<message::MessageWaypoint>(&text).unwrap();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}