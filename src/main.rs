use spacedust::apis::agents_api::get_my_agent;
use spacedust::apis::configuration::Configuration;
use spacedust::apis::default_api::register;
use spacedust::models::register_request::{Faction, RegisterRequest};
use struct_db::*;
use colored::*;

use entities::schemas;
use database::db;

mod generators;
mod database;
mod logger;
mod entities;

#[tokio::main]
async fn main() -> Result<()>{
    let mut lattice = db::init_db("lattice")?;
    /* db op example
    db::insert(&lattice, db::Agents{
        symbol: "TEST".to_string(),
        token: "AUTH_TOKEN".to_string()
    });
    let agent: db::Agents = db::read(&lattice, "TEST".to_string());
    println!("{:#?}", agent);
    */
    println!(">> {} <<", "BOOTING NEUROMORPHIC CORE".white());
    println!("> Instantiating SERAPH translation angel...");
    println!("!> {}", "ARCHANGEL.RAZIEL is ONLINE. Greetings, user. I will now run some post-boot protocols.".yellow());

    logger::log(&"Hi :3".to_string(), logger::AlertType::ALERT, true);
    logger::log(&"hehe :3".to_string(), logger::AlertType::WARNING, false);

    // create config
    let mut config = Configuration::new();
    // register request
    let id: String = generators::generate_hex_ID();
    let user_agent = format!("VIRTUE-{}", id); // agent name with ARCHANGEL + id
    
    let agent = schemas::Agents{
        symbol: user_agent.clone(),
        token: "idk".to_string()
    };

    // get bearer token
    let token = db::read::<schemas::Agents>(&lattice, "VIRTUE-C8DB26".to_string()).token;
    println!("{:#?}", token);
    config.bearer_access_token = Some(token.clone());
    // Create search object
    let search = schemas::Agents::new("VIRTUE-C8DB26".to_string(), "".to_string());
    // get data
    let results = search.get_data(&mut config).await.unwrap();
    println!("{:#?}", results);
    // get server status
    let status = schemas::Agents::get_server_status(&mut config).await.unwrap();
    println!("{:#?}", status);

    Ok(())
}