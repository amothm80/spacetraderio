use crate::apis::config;
use crate::apis::errors;
use crate::models::contract;
use crate::models::message;

pub async fn get_my_contracts(
    config: &config::Config,
) -> Result<Vec<contract::Contract>, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::GET,
        config.base_path.to_owned() + "/my/contracts",
    );
    reqbuilder = reqbuilder.bearer_auth(config.bearer_token.to_owned());
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let text = resp.text().await?;
    if !status.is_server_error() && !text.is_empty() {
        let json = serde_json::from_str::<message::MessageMyContracts>(&text)?;
        if json.error.code > 0 {
            Err(errors::STError::stapierror(json.error))
        } else {
            Ok(json.data)
        }
    } else {
        Err(errors::STError::stapierror(message::ErrorContent {
            message: String::from("No contract data"),
            symbol: String::from(""),
            code: 999,
        }))
    }
}

pub async fn get_contract_info(
    config: &config::Config,
    contract: String,
) -> Result<contract::Contract, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::GET,
        config.base_path.to_owned() + "/my/contracts/".to_owned().as_str() + contract.as_str(),
    );
    reqbuilder = reqbuilder.bearer_auth(config.bearer_token.to_owned());
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let text = resp.text().await?;
    if !status.is_server_error() && !text.is_empty() {
        let json = serde_json::from_str::<message::MessageContract>(&text)?;
        if json.error.code > 0 {
            Err(errors::STError::stapierror(json.error))
        } else {
            Ok(json.data)
        }
    } else {
        Err(errors::STError::stapierror(message::ErrorContent {
            message: String::from("No contract data"),
            symbol: String::from(""),
            code: 999,
        }))
    }
}

pub async fn accept_contract(
    config: &config::Config,
    contract: String,
) -> Result<contract::Contract, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::POST,
        config.base_path.to_owned()
            + "/my/contracts/".to_owned().as_str()
            + contract.to_owned().as_str()
            + "/accept",
    );
    reqbuilder = reqbuilder.bearer_auth(config.bearer_token.to_owned());
    reqbuilder = reqbuilder.header("Content-length", 0);
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let text = resp.text().await?;
    if !status.is_server_error() && !text.is_empty() {
        let json = serde_json::from_str::<message::MessageContractAcceptance>(&text)?;
        if json.error.code > 0 {
            Err(errors::STError::stapierror(json.error))
        } else {
            Ok(json.data.contract)
        }
    } else {
        Err(errors::STError::stapierror(message::ErrorContent {
            message: String::from("No contract acceptance data"),
            symbol: String::from(""),
            code: 999,
        }))
    }
}

pub async fn fulfill_contract(
    config: &config::Config,
    contract: String,
) -> Result<contract::Contract, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::POST,
        config.base_path.to_owned()
            + "/my/contracts/".to_owned().as_str()
            + contract.to_owned().as_str()
            + "/fulfill",
    );
    reqbuilder = reqbuilder.bearer_auth(config.bearer_token.to_owned());
    reqbuilder = reqbuilder.header("Content-length", 0);
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let text = resp.text().await?;
    if !status.is_server_error() && !text.is_empty() {
        let json = serde_json::from_str::<message::MessageContractFulfillment>(&text)?;
        if json.error.code > 0 {
            Err(errors::STError::stapierror(json.error))
        } else {
            Ok(json.data.contract)
        }
    } else {
        Err(errors::STError::stapierror(message::ErrorContent {
            message: String::from("No contract fulfillment data"),
            symbol: String::from(""),
            code: 999,
        }))
    }
}
