use spacedust::apis::agents_api::get_my_agent;
use spacedust::apis::configuration::Configuration;
use spacedust::apis::default_api::register;
use spacedust::models::register_request::{Faction, RegisterRequest};
use struct_db::*;
use colored::*;
use std::env;
use rhai::{Engine, EvalAltResult, AST};

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
mod tests;

#[tokio::main]
async fn main() -> Result<()>{
    // Dependencies for the program to inject into module functions
    let mut lattice = db::init_db("lattice")?; // database
    let mut engine = scripts::init_engine().unwrap(); // script engine
    let mut config = Configuration::new(); // config for spacedust api
    let mut limiter = rate::init_rate_limiter(&config); // rate limiter

    boot_log();

    tests::run_tests(&mut lattice, &mut engine, &mut config, &mut limiter).await;

    // register request [testing]
    let id: String = generators::generate_hex_ID();
    let user_agent = format!("VIRTUE-{}", id); // agent name with ARCHANGEL + id

    // get bearer token
    let token = db::read::<Agents>(&lattice, "VIRTUE-C8DB26".to_string()).token;
    config.bearer_access_token = Some(token.clone());
    // Create search object
    
    // get data
    // test rate limit by sending a lot of data
    // for count in 0..100{
    //     let results = search.get_data(&mut config, &limiter).await.unwrap();
    //     println!("{}: {:#?}",count, results);
    // }
    // args
    // Here we read the file path as a command line argument run it to the script engine
    
    Ok(())
}

fn handle_scripts(engine: &Engine){
    let args: Vec<_> = env::args().collect();
    let script_path = args[1].clone(); // this is the file path of the script to read
    if args.len() > 1{
        
    }else{
        // FIXME: no args, what do we do? 
    }
}

fn boot_log(){
    println!(">> {} <<", "BOOTING NEUROMORPHIC CORE".white());
    println!("> Instantiating SERAPH translation angel...");
    println!("!> {}", "ARCHANGEL.RAZIEL is ONLINE. Greetings, user. I will now run some post-boot protocols.".yellow());

    logger::log("ARCHANGEL.RAZIEL is ONLINE. Greetings, user. I will now run some post-boot protocols.", logger::AlertType::DEFAULT, true);
}