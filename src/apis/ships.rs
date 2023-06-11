use crate::apis::config;
use crate::apis::errors;
use crate::models::cooldown;
use crate::models::message;
use crate::models::message::ErrorContent;
use crate::models::message::MessageShipNavigationData;
use crate::models::message::MessageShipPurchaseData;
use crate::models::meta;
use crate::models::ship;
use crate::models::shipcargo;
use crate::models::shipnav;
use crate::models::shipnavflightmode;
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
) -> Result<message::MessageShipOrbitData, errors::STError> {
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
        Ok(json.data)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}

pub async fn chart_waypoint(
    config: &config::Config,
    ship_symbol: String,
) -> Result<message::MessageShipChartData, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::POST,
        config.base_path.to_owned() + "/my/ships/" + ship_symbol.as_str() + "/chart",
    );
    reqbuilder = reqbuilder.header("Content-length", 0);
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let text = resp.text().await?;
    //println!("{:#?}", text);
    //let json = resp.json::<message::Message>().await?;
    let json = serde_json::from_str::<message::MessageShipChart>(&text).unwrap();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}

pub async fn get_cooldown(
    config: &config::Config,
    ship_symbol: String,
) -> Result<message::MessageShipCooldown, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::GET,
        config.base_path.to_owned() + "/my/ships/" + ship_symbol.as_str() + "/cooldown",
    );
    reqbuilder = reqbuilder.bearer_auth(config.bearer_token.to_owned());
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let text = resp.text().await?;
    //println!("{:#?}",text);
    //let json = resp.json::<message::Message>().await?;
    let json = serde_json::from_str::<message::MessageShipCooldown>(&text).unwrap();
    if !status.is_client_error() && !status.is_server_error() {
        if status.as_u16() == 204 {
            let message = message::MessageShipCooldown {
                data: cooldown::Cooldown::default(),
                no_cooldown: String::from("No cooldown"),
                meta: meta::Meta::default(),
                error: ErrorContent::default(),
            };
            Ok(message)
        } else {
            Ok(json)
        }
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}

pub async fn dock_ship(
    config: &config::Config,
    ship_symbol: String,
) -> Result<message::MessageShipOrbitData, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::POST,
        config.base_path.to_owned() + "/my/ships/" + ship_symbol.as_str() + "/dock",
    );
    reqbuilder = reqbuilder.header("Content-length", 0);
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let text = resp.text().await?;
    //println!("{:#?}", text);
    //let json = resp.json::<message::Message>().await?;
    let json = serde_json::from_str::<message::MessageShipOrbit>(&text).unwrap();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}

pub async fn survey_waypoint(
    config: &config::Config,
    ship_symbol: String,
) -> Result<message::MessageShipSurveyData, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::POST,
        config.base_path.to_owned() + "/my/ships/" + ship_symbol.as_str() + "/survey",
    );
    reqbuilder = reqbuilder.header("Content-length", 0);
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let text = resp.text().await?;
    //println!("{:#?}", text);
    //let json = resp.json::<message::Message>().await?;
    let json = serde_json::from_str::<message::MessageShipSurvey>(&text).unwrap();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}

pub async fn extract_resource(
    config: &config::Config,
    ship_symbol: String,
) -> Result<message::MessageShipSurveyData, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::POST,
        config.base_path.to_owned() + "/my/ships/" + ship_symbol.as_str() + "/extract",
    );
    reqbuilder = reqbuilder.header("Content-length", 0);
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let text = resp.text().await?;
    //println!("{:#?}", text);
    //let json = resp.json::<message::Message>().await?;
    let json = serde_json::from_str::<message::MessageShipSurvey>(&text).unwrap();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}

pub async fn jettison_cargo(
    config: &config::Config,
    ship_symbol: String,
    resource_symbol: String,
    units: i64,
) -> Result<message::MessageShipJettisonData, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::POST,
        config.base_path.to_owned() + "/my/ships/" + ship_symbol.as_str() + "/jettison",
    );
    let payload = format!("{{\"symbol\":\"{resource_symbol}\",\"units\":\"{units}\"}}");
    reqbuilder = reqbuilder.header("Content-length", payload.len());
    reqbuilder = reqbuilder.header("Content-type", "application/json");
    reqbuilder = reqbuilder.body(payload);
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let text = resp.text().await?;
    //println!("{:#?}", text);
    //let json = resp.json::<message::Message>().await?;
    let json = serde_json::from_str::<message::MessageShipJettison>(&text).unwrap();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}

pub async fn ship_jump(
    config: &config::Config,
    ship_symbol: String,
    system_symbol: String,
) -> Result<message::MessageShipJumpData, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::POST,
        config.base_path.to_owned() + "/my/ships/" + ship_symbol.as_str() + "/jump",
    );
    let payload = format!("{{\"systemSymbol\":\"{system_symbol}\"}}");
    reqbuilder = reqbuilder.header("Content-length", payload.len());
    reqbuilder = reqbuilder.header("Content-type", "application/json");
    reqbuilder = reqbuilder.body(payload);
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let text = resp.text().await?;
    //println!("{:#?}", text);
    //let json = resp.json::<message::Message>().await?;
    let json = serde_json::from_str::<message::MessageShipJump>(&text).unwrap();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}

pub async fn ship_update_nav(
    config: &config::Config,
    ship_symbol: String,
    flight_mode: String,
) -> Result<shipnav::ShipNav, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::PATCH,
        config.base_path.to_owned() + "/my/ships/" + ship_symbol.as_str() + "/nav",
    );
    let payload = format!("{{\"flightMode\":\"{flight_mode}\"}}");
    reqbuilder = reqbuilder.header("Content-length", payload.len());
    reqbuilder = reqbuilder.header("Content-type", "application/json");
    reqbuilder = reqbuilder.body(payload);
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let text = resp.text().await?;
    //println!("{:#?}", text);
    //let json = resp.json::<message::Message>().await?;
    let json = serde_json::from_str::<message::MessageShipNav>(&text).unwrap();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}

pub async fn ship_warp(
    config: &config::Config,
    ship_symbol: String,
    waypoint_symbol: String,
) -> Result<message::MessageShipWarpData, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::POST,
        config.base_path.to_owned() + "/my/ships/" + ship_symbol.as_str() + "/warp",
    );
    let payload = format!("{{\"waypointSymbol\":\"{waypoint_symbol}\"}}");
    reqbuilder = reqbuilder.header("Content-length", payload.len());
    reqbuilder = reqbuilder.header("Content-type", "application/json");
    reqbuilder = reqbuilder.body(payload);
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let text = resp.text().await?;
    //println!("{:#?}", text);
    //let json = resp.json::<message::Message>().await?;
    let json = serde_json::from_str::<message::MessageShipWarp>(&text).unwrap();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}

pub async fn sell_cargo(
    config: &config::Config,
    ship_symbol: String,
    cargo_symbol: String,
    units: i64,
) -> Result<message::MessageShipSellCargoData, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::POST,
        config.base_path.to_owned() + "/my/ships/" + ship_symbol.as_str() + "/sell",
    );
    let payload = format!("{{\"symbol\":\"{cargo_symbol}\",\"units\":\"{units}\"}}");
    reqbuilder = reqbuilder.header("Content-length", payload.len());
    reqbuilder = reqbuilder.header("Content-type", "application/json");
    reqbuilder = reqbuilder.body(payload);
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let text = resp.text().await?;
    //println!("{:#?}", text);
    //let json = resp.json::<message::Message>().await?;
    let json = serde_json::from_str::<message::MessageShipSellCargo>(&text).unwrap();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}

pub async fn scan_systems(
    config: &config::Config,
    ship_symbol: String,
    cargo_symbol: String,
    units: i64,
) -> Result<message::MessageShipScanSystemsData, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::POST,
        config.base_path.to_owned() + "/my/ships/" + ship_symbol.as_str() + "/scan/systems",
    );
    reqbuilder = reqbuilder.header("Content-length", 0);
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let text = resp.text().await?;
    //println!("{:#?}", text);
    //let json = resp.json::<message::Message>().await?;
    let json = serde_json::from_str::<message::MessageShipScanSystems>(&text).unwrap();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}
