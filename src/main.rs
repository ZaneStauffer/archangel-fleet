use spacedust::apis::agents_api::get_my_agent;
use spacedust::apis::configuration::Configuration;
use spacedust::apis::default_api::register;
use spacedust::models::register_request::{Faction, RegisterRequest};
use struct_db::*;
use colored::*;

use database::db;
mod generators;
mod database;
mod logger;

#[tokio::main]
async fn main() -> Result<()>{
    let mut lattice = db::init_db("lattice")?;
    let rd = lattice.read_transaction().unwrap();
    let mut tables = rd.tables();

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
    // let register_request = RegisterRequest::new(Faction::Quantum, user_agent.clone());

    logger::log(&format!("{} has been instantiated. Storing agent signature in LATTICE.", user_agent),
        logger::AlertType::ALERT,
        true
    );
    // register agent
    /*
    let register_response = register(&config, Some(register_request)).await;
    match register_response {
        Ok(res) => {
            println!("{:#?}", res);
            // here: update JSON with symbol and token (and eventually user)
            config.bearer_access_token = Some(res.data.token);
        }
        Err(e) => {
            panic!("{:#?}", e);
        }
    }
    */

    Ok(())
}