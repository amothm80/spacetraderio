pub mod apis;
pub mod models;

use apis::agent;
//use apis::config::Config;
use apis::contracts;
use apis::errors;
use apis::factions;
use apis::ships;
use apis::systems;

pub async fn process_commands(
    args: Vec<String>,
    conf: &apis::config::Config,
) -> Result<String, errors::STError> {
    match args[1].as_str() {
        "register" => Ok(format!(
            "{}",
            agent::register(
                conf,
                args[2].to_owned(), // "COSMIC","VOID", "GALACTIC","QUANTUM","DOMINION"
                args[3].to_owned(), //3 to 14 characters
                args[4].to_owned()
            )
            .await?
        )),
        "agent" => Ok(format!("{}", agent::get_my_agent(conf).await?)),
        "faction" => match args[2].as_str() {
            "all" => {
                let factions = factions::get_factions(conf).await?;
                let mut disp = String::from("");
                for faction in factions {
                    disp = disp.to_owned() + format!("{}\n", faction).as_str();
                }
                Ok(disp)
            }
            _ => Ok(format!(
                "{}",
                factions::get_faction_info(conf, args[2].to_owned()).await?
            )),
        },
        "contract" => match args[2].as_str() {
            "all" => {
                let contracts = contracts::get_my_contracts(conf).await?;
                let mut disp = String::from("");
                for contract in contracts {
                    disp = disp.to_owned() + format!("{}\n", contract).as_str();
                }
                Ok(disp)
            }
            "accept" => Ok(format!(
                "{}\nACCEPTED",
                contracts::accept_contract(conf, args[3].to_owned()).await?
            )),
            // match agent::accept_contract(conf, args[3].to_owned()).await {
            //     Ok(s) => Ok(format!("{}\nACCEPTED", s),
            //     Err(s) => handle_errors(s),
            // },
            "fulfill" => Ok(format!(
                "{}\nFULILLED",
                contracts::fulfill_contract(conf, args[3].to_owned()).await?
            )),
            _ => Ok(format!(
                "{}",
                contracts::get_contract_info(conf, args[2].to_owned()).await?
            )),
        },
        "ships" => match args[2].as_str() {
            "buy" => Ok(format!(
                "{}\nBOUGHT\n",
                ships::buy_ship(conf, args[3].to_owned(), args[4].to_owned()).await?
            )),
            "navigate" => Ok(format!(
                "{}\n",
                ships::navigate_ship(conf, args[3].to_owned(), args[4].to_owned()).await?
            )),
            "cargo" => Ok(format!(
                "{}",
                ships::get_my_ship_cargo(conf, args[3].to_owned()).await?
            )),
            "orbit" => Ok(format!(
                "{}",
                ships::orbit_my_ship(conf, args[3].to_owned()).await?
            )),
            "refine" => Ok(format!(
                "{}",
                ships::refine_materials(conf, args[3].to_owned(), args[4].to_owned()).await?
            )),
            "chart" => Ok(format!(
                "{}",
                ships::chart_waypoint(conf, args[3].to_owned()).await?
            )),
            "cooldown" => Ok(format!(
                "{}",
                ships::get_cooldown(conf, args[3].to_owned()).await?
            )),
            "dock" => Ok(format!(
                "{}",
                ships::dock_ship(conf, args[3].to_owned()).await?
            )),
            "survey" => Ok(format!(
                "{}",
                ships::survey_waypoint(conf, args[3].to_owned()).await?
            )),
            "extract" => Ok(format!(
                "{}",
                ships::extract_resource(conf, args[3].to_owned()).await?
            )),
            "jettison" => Ok(format!(
                "{}",
                ships::jettison_cargo(
                    conf,
                    args[3].to_owned(),
                    args[4].to_owned(),
                    args[5].to_owned()
                )
                .await?
            )),
            "jump" => Ok(format!(
                "{}",
                ships::ship_jump(conf, args[3].to_owned(), args[4].to_owned()).await?
            )),
            "updatenav" => Ok(format!(
                "{}",
                ships::ship_update_nav(conf, args[3].to_owned(), args[4].to_owned()).await?
            )),
            "warp" => Ok(format!(
                "{}",
                ships::ship_warp(conf, args[3].to_owned(), args[4].to_owned()).await?
            )),
            "sell" => Ok(format!(
                "{}",
                ships::sell_cargo(
                    conf,
                    args[3].to_owned(),
                    args[4].to_owned(),
                    args[5].to_owned()
                )
                .await?
            )),
            "scansystems" => Ok(format!(
                "{}",
                ships::scan_systems(conf, args[3].to_owned()).await?
            )),
            "scanwaypoints" => Ok(format!(
                "{}",
                ships::scan_waypoints(conf, args[3].to_owned()).await?
            )),
            "scanships" => Ok(format!(
                "{}",
                ships::scan_ships(conf, args[3].to_owned()).await?
            )),
            "refuel" => Ok(format!(
                "{}",
                ships::refuel_ship(conf, args[3].to_owned()).await?
            )),
            "purchase" => Ok(format!(
                "{}",
                ships::purchase_cargo(
                    conf,
                    args[3].to_owned(),
                    args[4].to_owned(),
                    args[5].to_owned()
                )
                .await?
            )),
            "transfer" => Ok(format!(
                "{}",
                ships::transfer_cargo(
                    conf,
                    args[3].to_owned(),
                    args[4].to_owned(),
                    args[5].to_owned(),
                    args[6].to_owned()
                )
                .await?
            )),
            _ => {
                let ships = ships::get_my_ships(conf).await?;
                let mut disp = String::from("");
                for ship in ships {
                    disp = disp.to_owned() + format!("{}\n\n", ship).as_str();
                }
                Ok(disp)
            }
        },
        "system" => match args[2].as_str() {
            "waypoint" => match args[3].as_str() {
                "info" => Ok(format!(
                    "{}",
                    systems::get_system_waypoint_info(conf, args[4].to_owned(), args[5].to_owned())
                        .await?
                )),
                "market" => Ok(format!(
                    "{}",
                    systems::get_waypoint_market(conf, args[4].to_owned(), args[5].to_owned())
                        .await?
                )),
                "shipyard" => {
                    Ok(format!(
                        "{}",
                        systems::get_waypoint_shipyard(
                            //cargo run system waypoint shipyard X1-XT43 X1-XT43-27307E
                            conf,
                            args[4].to_owned(),
                            args[5].to_owned()
                        )
                        .await?
                    ))
                }
                "jumpgate" => {
                    Ok(format!(
                        "{}",
                        systems::get_waypoint_jumpgate(
                            //cargo run system waypoint shipyard X1-XT43 X1-XT43-27307E
                            conf,
                            args[4].to_owned(),
                            args[5].to_owned()
                        )
                        .await?
                    ))
                }
                _ => {
                    let waypoints = systems::get_system_waypoints(conf, args[4].to_owned()).await?;
                    let mut disp = String::from("");
                    for wp in waypoints {
                        disp = disp.to_owned() + format!("{}", wp).as_str();
                    }
                    Ok(disp)
                }
            },
            _ => {
                let systems = systems::get_systems(conf).await?;
                let mut disp = String::from("");
                for system in systems {
                    disp = disp.to_owned() + format!("{}\n", system).as_str();
                }
                Ok(disp)
            }
        },
        _ => Ok(String::from("invalid argument")),
    }

    // Ok(String::from("invalid argument"))
}

pub fn handle_errors(e: errors::STError) {
    match e {
        errors::STError::reqwesterror(e) => println!("{}", e),
        errors::STError::serdejerror(e) => println!("{}", e),
        errors::STError::stapierror(e) => println!("{}", e),
        errors::STError::stgeneralerror => println!("{}", e),
    }
}
