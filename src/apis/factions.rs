use crate::models::agent;
use crate::apis::config;
use reqwest::Error;

pub async fn get_my_factions(config: &config::Config)->Result<agent::Agent,Error>{

    let client = &config.client;
    let mut reqbuilder = client.request(reqwest::Method::GET, config.base_path.to_owned() + "/my/agent" );
    reqbuilder = reqbuilder.bearer_auth(config.bearer_token.to_owned());
    let req = reqbuilder.build()?;
    let resp = client.execute(req).await?;
    let status = resp.status();
    //let content = resp.text().await?;
    let agent = resp.json::<agent::Data>().await?;


    ////let resp = client.get(ST_API_URL).send().await?.json::<HashMap<String, Value>>().await?;
    ////let resp = client.get(ST_API_URL.to_owned()+ST_API_AGENT).bearer_auth(token).send().await?;//.json::<HashMap<String, Value>>().await?;
    //let resp = client
    //    .get("https://api.spacetraders.io/v2/my/contracts/clhi30h5t10hfs60dompcbn9p")
    //    .bearer_auth(token)
    //    .send()
    //    .await?;
    ////let json = resp.json::<RootResp>().await?;
    //println!("{:#?}", resp.text().await?);
    ////println!("{:#?}", resp.json::<models::contract::Contract>().await?);
    ////let agent = resp.json::<models::agent::Data>().await?.data;
    ////println!("{:#?}", agent);
    Ok(agent.data)
}