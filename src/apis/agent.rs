use crate::apis::config;
use crate::apis::errors;
use crate::models::agent;
use crate::models::message;

pub async fn register(
    config: &config::Config,
    faction: String, // "COSMIC","VOID", "GALACTIC","QUANTUM","DOMINION"
    symbol: String,  //3 to 14 characters
    email: String,
) -> Result<message::MessageAgentRegisterData, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::POST,
        config.base_path.to_owned() + "/register",
    );
    let payload =
        format!("{{\"faction\":\"{faction}\",\"symbol\":\"{symbol}\",\"email\":\"{email}\"}}");
    reqbuilder = reqbuilder.header("Content-length", payload.len());
    reqbuilder = reqbuilder.header("Content-type", "application/json");
    reqbuilder = reqbuilder.body(payload);
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let text = resp.text().await?;
    //println!("{:#?}", text);
    //let json = resp.json::<message::Message>().await?;
    let json = serde_json::from_str::<message::MessageAgentRegister>(&text)?;
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}

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
    let json = resp.json::<message::MessageMyAgent>().await?;
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}
