use spacedust::apis::agents_api::get_my_agent;
use spacedust::apis::configuration::Configuration;
use spacedust::apis::default_api::register;
use spacedust::models::register_request::{Faction, RegisterRequest};
use struct_db::*;
use colored::*;

use entities::schemas;
// import all entity schema
use entities::{agent::*, ship::*};
use database::db;

mod generators;
mod database;
mod logger;
mod entities;
mod rate;
mod scripts;

#[tokio::main]
async fn main() -> Result<()>{
    let mut lattice = db::init_db("lattice")?;
    //let mut engine = Engine::new(); // create rhai scripting engine
    // TODO: do RHAI bindings here
    //let result = engine.eval_file::<i64>("scripts/test.rhai".into()).unwrap();
    
    println!(">> {} <<", "BOOTING NEUROMORPHIC CORE".white());
    println!("> Instantiating SERAPH translation angel...");
    println!("!> {}", "ARCHANGEL.RAZIEL is ONLINE. Greetings, user. I will now run some post-boot protocols.".yellow());

    logger::log(&"Hi :3".to_string(), logger::AlertType::ALERT, true);
    logger::log(&"hehe :3".to_string(), logger::AlertType::WARNING, false);

    // create config
    let mut config = Configuration::new();

    // create rate limiter
    let mut limiter = rate::init_rate_limiter(&config);
    println!("{:#?} tokens in the bucket (:", limiter.bucket.tokens());

    // register request [testing]
    let id: String = generators::generate_hex_ID();
    // generate agent ID [testing]
    let user_agent = format!("VIRTUE-{}", id); // agent name with ARCHANGEL + id
    
    let agent = Agents{
        symbol: user_agent.clone(),
        token: "idk".to_string()
    };

    // get bearer token
    let token = db::read::<Agents>(&lattice, "VIRTUE-C8DB26".to_string()).token;
    config.bearer_access_token = Some(token.clone());
    // Create search object
    let search = Agents::new("VIRTUE-C8DB26".to_string(), "".to_string());
    // get data
    // test rate limit by sending a lot of data
    // for count in 0..100{
    //     let results = search.get_data(&mut config, &limiter).await.unwrap();
    //     println!("{}: {:#?}",count, results);
    // }
    // get server status
    // let status = schemas::Agents::get_server_status(&mut config).await.unwrap();
    // println!("{:#?}", status);

    Ok(())
}