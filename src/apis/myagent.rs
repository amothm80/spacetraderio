use crate::apis::config;
use crate::apis::errors;
use crate::models::agent;
use crate::models::message;
use reqwest::Error;

pub async fn get_my_agent(config: &config::Config) -> Result<agent::Agent, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::GET,
        config.base_path.to_owned() + "/my/agent",
    );
    reqbuilder = reqbuilder.bearer_auth(config.bearer_token.to_owned());
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;

    match resp.json::<message::Message>().await? {
        message::Message::data(d) => match d {
            message::Data::agent(a) => Ok(a), //println!("{:#?}",a.accountId),
            //(_) => println!("{:#?}","error"),
            (_) => Err(errors::STError::stgeneralerror),
        },
        //message::Message::error(e) => println!("{:#?}",e),
        message::Message::error(e) => Err(errors::STError::stapierror(e)),
    }
}

//println!("status: {:#?} , {:#?}",resp.status(), resp.text().await?);
//println!("status: {:#?} , {:#?}",resp.status(), resp.json::<message::Message>().await?);
//let status = resp.status();
// if !status.is_client_error() && !status.is_server_error() {
//     Ok(resp.json::<agent::Data>().await?.data)
// }else{

// }
// match resp.error_for_status_ref(){
//     Ok(_res) => {Ok(resp.json::<agent::Data>().await?.data)},
//     Err(err) => {Err(errors::STError::reqwesterror(err))},
// }
//Ok(agent::Agent { accountId: String::from(""), symbol: String::from(""), headquarters: String::from(""), credits: 0 })
