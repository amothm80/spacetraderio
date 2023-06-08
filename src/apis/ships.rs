use crate::apis::config;
use crate::apis::errors;
use crate::models::agent;
use crate::models::message;
use crate::models::ship;
use crate::models::shipfuel::ShipFuel;
use crate::models::shipnav;
use crate::models::shipyardtransaction;
//use reqwest::Error;

pub async fn get_my_ships(config: &config::Config) -> Result<Vec<ship::Ship>, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::GET,
        config.base_path.to_owned() + "/my/ships",
    );
    reqbuilder = reqbuilder.bearer_auth(config.bearer_token.to_owned());
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let text = resp.text().await?;
    //println!("{:#?}",text);
    //let json = resp.json::<message::Message>().await?;
    let json = serde_json::from_str::<message::MessageMyShips>(&text).unwrap();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}

pub async fn buy_ship(
    config: &config::Config,
    shipType: String,
    waypointSymbol: String,
    //) -> Result<message::MessageShipPurchaseData, errors::STError> {
) -> Result<
    (
        agent::Agent,
        ship::Ship,
        shipyardtransaction::ShipyardTransaction,
    ),
    errors::STError,
> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::POST,
        config.base_path.to_owned() + "/my/ships",
    );
    reqbuilder = reqbuilder.bearer_auth(config.bearer_token.to_owned());
    let payload =
        format!("{{\"shipType\":\"{shipType}\",\"waypointSymbol\":\"{waypointSymbol}\"}}");
    reqbuilder = reqbuilder.header("Content-length", payload.len());
    reqbuilder = reqbuilder.header("Content-type", "application/json");
    reqbuilder = reqbuilder.body(payload);
    //reqbuilder = reqbuilder.
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let text = resp.text().await?;
    //println!("{:#?}",text);
    //let json = resp.json::<message::Message>().await?;
    let json = serde_json::from_str::<message::MessageShipPurchase>(&text).unwrap();
    if !status.is_client_error() && !status.is_server_error() {
        Ok((json.data.agent, json.data.ship, json.data.transaction))
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}

pub async fn navigate_ship(
    config: &config::Config,
    shipSymbol: String,
    waypointSymbol: String,
) -> Result<
    (
        ShipFuel,
        shipnav::ShipNav,
        shipyardtransaction::ShipyardTransaction,
    ),
    errors::STError,
> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::POST,
        config.base_path.to_owned() + "/my/ships/" + shipSymbol.as_str() + "/navigate",
    );
    reqbuilder = reqbuilder.bearer_auth(config.bearer_token.to_owned());
    let payload = format!("{{\"waypointSymbol\":\"{waypointSymbol}\"}}");
    reqbuilder = reqbuilder.header("Content-length", payload.len());
    reqbuilder = reqbuilder.header("Content-type", "application/json");
    reqbuilder = reqbuilder.body(payload);
    //reqbuilder = reqbuilder.
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let text = resp.text().await?;
    //println!("{:#?}",text);
    //let json = resp.json::<message::Message>().await?;
    let json = serde_json::from_str::<message::MessageShipNavigation>(&text).unwrap();
    if !status.is_client_error() && !status.is_server_error() {
        Ok((json.data.fuel, json.data.shipnav, json.data.transaction))
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}
