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
    let json = resp.json::<message::MessageMyContracts>().await?;
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data)
    } else {
        Err(errors::STError::stapierror(json.error))
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
    let json = resp.json::<message::MessageContract>().await?;
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data)
    } else {
        Err(errors::STError::stapierror(json.error))
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
    let json = resp.json::<message::MessageContractAcceptance>().await?;
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data.contract)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}

pub async fn deliver_contract(
    config: &config::Config,
    contract: String,
    ship_symbol: String,
    trade_symbol: String,
    units: i64,
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
    let payload =
        format!("{{\"shipSymbol\":\"{ship_symbol}\",\"tradeSymbol\":\"{trade_symbol}\",\"units\":\"{units}\"}}");
    reqbuilder = reqbuilder.header("Content-length", payload.len());
    reqbuilder = reqbuilder.header("Content-type", "application/json");
    reqbuilder = reqbuilder.body(payload);
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    let json = resp.json::<message::MessageContractFulfillment>().await?;
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data.contract)
    } else {
        Err(errors::STError::stapierror(json.error))
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
    let json = resp.json::<message::MessageContractFulfillment>().await?;
    if !status.is_client_error() && !status.is_server_error() {
        Ok(json.data.contract)
    } else {
        Err(errors::STError::stapierror(json.error))
    }
}
