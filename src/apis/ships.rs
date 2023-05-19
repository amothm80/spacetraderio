use crate::apis::config;
use crate::apis::errors;
use crate::models::ship;
use crate::models::message;
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