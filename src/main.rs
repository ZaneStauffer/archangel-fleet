use spacedust::apis::agents_api::get_my_agent;
use spacedust::apis::configuration::Configuration;
use spacedust::apis::default_api::register;
use spacedust::models::register_request::{Faction, RegisterRequest};
use struct_db::*;
use colored::*;
use std::env;
use std::path::{Path, PathBuf};
use rhai::{Engine, EvalAltResult, AST};
use lazy_static::lazy_static;

use entities::schemas;
// import all entity schema
use entities::{agent::*, ship::*};
use database::db;

use crate::rate::RateLimiter;

mod generators;
mod database;
mod logger;
mod entities;
mod rate;
mod scripts;
mod tests;

pub mod statics{
    use lazy_static::lazy_static;
    use spacedust::apis::configuration::Configuration;
    use crate::rate;
    use crate::database::db;
    use struct_db::*;

    /*
    JOHN 1:9
    If we confess our sins, he is faithful and just to forgive us our sins, and to cleanse us from all unrighteousness.
    */
    lazy_static!{
        pub static ref Lattice: Db = {
            db::init_db("lattice").unwrap()
        };
        pub static ref Config: Configuration = {
            use crate::database::db;
            use crate::entities::{agent::Agents};

            let mut _config = Configuration::new();

            let agent_symbol: String = "VIRTUE-C8DB26".to_string();
            let token = db::read::<Agents>(&Lattice, agent_symbol).token;
            _config.bearer_access_token = token.clone();
            _config
            // set token
            // let token = db::read::<Agents>(&db, agent_symbol.to_string()).token;
        };
        pub static ref Limiter: rate::RateLimiter = rate::init_rate_limiter(&Config);
    }
}

#[tokio::main]
async fn main() -> Result<()>{
    // Dependencies for the program to inject into module functions
    logger::log("Initializing dependencies...", logger::AlertType::DEFAULT, false);
    let mut lattice = db::init_db("lattice")?; // database
    let mut engine = scripts::init_engine().unwrap(); // script engine
    //let mut config = Configuration::new(); // config for spacedust apiate limiter

    // use statics for the config and limiter
    // global variables are bad but this is the only way to do it i think for now

    boot_log();

    let agent = "VIRTUE-C8DB26"; // TODO: agent token switching
   
    //set_token(&lattice, &statics::config, agent); // Sets the agent auth token for spacedust api
    logger::log("I am now running your test functions...", logger::AlertType::DEFAULT, false);
    tests::run_tests(&mut lattice, &mut engine, &statics::Config, &statics::Limiter).await;

    logger::log("Now executing scripts...", logger::AlertType::DEFAULT, false);
    handle_scripts(&engine);
    
    Ok(())
}

// TODO: file parameter
fn handle_scripts(engine: &Engine){
// cargo run -- ./scripts/test.rhai
    let args: Vec<_> = env::args().collect();
    if args.len() > 1{
        let script_path = args[1].clone(); // this is the file path of the script to read
        let arg_script = scripts::read_script(engine, script_path.clone()).unwrap();
        logger::log(format!("Executing script: {}", script_path.clone()).as_str(), logger::AlertType::DEFAULT, true);
        // make this async? 
        let mut result = scripts::execute(engine, arg_script).unwrap();
        logger::log(format!("Script result: {}", result).as_str(), logger::AlertType::DEFAULT, true);
    }else{
        // FIXME: no args, what do we do? 
        logger::log("No script path provided. Please provide a script path as an argument to the program.", logger::AlertType::ALERT, false);
    }
}

fn boot_log(){
    println!(">> {} <<", "BOOTING NEUROMORPHIC CORE".white());
    println!("> Instantiating SERAPH translation angel...");
    println!("!> {}", "ARCHANGEL.RAZIEL is ONLINE. Greetings, user. I will now run some post-boot protocols.".yellow());

    logger::log("ARCHANGEL.RAZIEL is ONLINE. Greetings, user. I will now run some post-boot protocols.", logger::AlertType::DEFAULT, true);
}

// fn set_token(db: &Db, config: &Configuration, agent_symbol: &str){
//     // get bearer token
//     let token = db::read::<Agents>(&db, agent_symbol.to_string()).token;
//     config.bearer_access_token = token.clone();
// }