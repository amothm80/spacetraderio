use crate::apis::config;
use crate::apis::errors;
use crate::models::message;
use crate::models::message::MessageShipNavigationData;
use crate::models::message::MessageShipPurchaseData;
use crate::models::ship;
use crate::models::shipcargo;
use crate::models::shipnav;
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
    ship_type: String,
    waypoint_symbol: String,
) -> Result<MessageShipPurchaseData, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::POST,
        config.base_path.to_owned() + "/my/ships",
    );
    reqbuilder = reqbuilder.bearer_auth(config.bearer_token.to_owned());
    let payload =
        format!("{{\"shipType\":\"{ship_type}\",\"waypointSymbol\":\"{waypoint_symbol}\"}}");
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
        Ok(json.data)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}

pub async fn navigate_ship(
    config: &config::Config,
    ship_symbol: String,
    waypoint_symbol: String,
) -> Result<MessageShipNavigationData, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::POST,
        config.base_path.to_owned() + "/my/ships/" + ship_symbol.as_str() + "/navigate",
    );
    reqbuilder = reqbuilder.bearer_auth(config.bearer_token.to_owned());
    let payload = format!("{{\"waypointSymbol\":\"{waypoint_symbol}\"}}");
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
        Ok(json.data)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}

pub async fn get_my_ship_cargo(
    config: &config::Config,
    ship_symbol: String,
) -> Result<shipcargo::ShipCargo, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::GET,
        config.base_path.to_owned() + "/my/ships/" + ship_symbol.as_str() + "/cargo",
    );
    reqbuilder = reqbuilder.bearer_auth(config.bearer_token.to_owned());
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let text = resp.text().await?;
    //println!("{:#?}",text);
    //let json = resp.json::<message::Message>().await?;
    let json = serde_json::from_str::<message::MessageShipCargo>(&text).unwrap();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}

pub async fn orbit_my_ship(
    config: &config::Config,
    ship_symbol: String,
) -> Result<shipnav::ShipNav, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::POST,
        config.base_path.to_owned() + "/my/ships/" + ship_symbol.as_str() + "/orbit",
    );
    reqbuilder = reqbuilder.bearer_auth(config.bearer_token.to_owned());
    reqbuilder = reqbuilder.header("Content-length", 0);
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let text = resp.text().await?;
    //println!("{:#?}", text);
    //let json = resp.json::<message::Message>().await?;
    let json = serde_json::from_str::<message::MessageShipOrbit>(&text).unwrap();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data.nav)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}

pub async fn refine_materials(
    config: &config::Config,
    ship_symbol: String,
    produce: String,
) -> Result<shipnav::ShipNav, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::POST,
        config.base_path.to_owned() + "/my/ships/" + ship_symbol.as_str() + "/orbit",
    );
    let payload = format!("{{\"produce\":\"{produce}\"}}");
    reqbuilder = reqbuilder.header("Content-length", payload.len());
    reqbuilder = reqbuilder.header("Content-type", "application/json");
    reqbuilder = reqbuilder.body(payload);
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let text = resp.text().await?;
    //println!("{:#?}", text);
    //let json = resp.json::<message::Message>().await?;
    let json = serde_json::from_str::<message::MessageShipOrbit>(&text).unwrap();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data.nav)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}
