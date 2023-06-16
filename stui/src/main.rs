use stapi::*;

// use apis::agent;
// use apis::contracts;
// use apis::errors;
// use apis::factions;
// use apis::ships;
// use apis::systems;
use stapi::apis::config::Config;
use std::env;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();
    let token = fs::read_to_string("token.txt")?;
    let mut conf = Config::new();
    conf.bearer_token = token;
    match process_commands(args, &conf).await {
        Ok(s) => {
            println!("{}", s)
        }
        Err(e) => handle_errors(e),
    }
    //let agent = agent::get_my_agent(conf).await;
    //let mut contracts: Vec<models::contract::Contract> = vec![];
    Ok(())
}
