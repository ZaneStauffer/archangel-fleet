use struct_db::*;
use colored::*;
use std::env;
use rhai::{Engine};
use database::db;

mod generators;
mod database;
mod logger;
mod entities;
mod rate;
mod scripts;
mod tests;

#[allow(non_upper_case_globals)]

pub mod statics{
    use lazy_static::lazy_static;
    use spacedust::apis::configuration::Configuration;
    use crate::rate;
    use crate::database::db;
    use struct_db::*;
    use std::sync::Mutex;

    /*
    JOHN 1:9
    If we confess our sins, he is faithful and just to forgive us our sins, and to cleanse us from all unrighteousness.
    */
    lazy_static!{
        pub static ref Lattice: Db = {
            db::init_db("data").unwrap()
        };
        pub static ref Config: Mutex<Configuration> = {
            let mut _config = Configuration::new();
            /*
            let agent_symbol: String = "VIRTUE-C8DB26".to_string();
            println!("now reading token for agent: {}", agent_symbol.clone());
            // vvvvvvvvvvvvvvvvvvvvvvvvv FIXME:
            let token = db::read::<Agents>(&Lattice, agent_symbol).token;
            // ^^^^^^^^^^^^^^^^^^^^^^^^^
            println!("token: {}", token.clone());
            */
            let _test_token = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZGVudGlmaWVyIjoiQU5HRUwtMDEiLCJ2ZXJzaW9uIjoidjIiLCJyZXNldF9kYXRlIjoiMjAyMy0wNi0wMyIsImlhdCI6MTY4NTkxODc0NSwic3ViIjoiYWdlbnQtdG9rZW4ifQ.yjlS9DEWZ9AJaqO0tyjt-kJjz2RB-m85LIf28WdhIeJp4jU4ynjKu-3StgCA6kLgbDOevRR_xS_TXZ75Z97B6EVrWd4vrXb9ci_Zpm8LbdOC6Yk_Aewxh8WYnc4XaJvv1psdcmGmLyQH8fHl1fjumGVlOM3GVf4M5lKzT4SlX4TAVVzs7F_vFpbfHZlGewKiRdOmxhlzuJoMRk4pODDxKpL7a0UzOdr8rq4hNjRTjOLZWrQQw_2daCE6eWGNaOPmX0RE4aoLShP4YO4k_G4rB6GcYYzJgtJydnQ5zbEZC7t017lLGRdqwgtMADqIUKfiu0zZC8J7RMIQy9kAgVbj7g";
            _config.bearer_access_token = Some(_test_token.to_string().clone());
            Mutex::new(_config)
            // set token
            // let token = db::read::<Agents>(&db, agent_symbol.to_string()).token;
        };
        pub static ref Limiter: rate::RateLimiter = rate::init_rate_limiter(&Config);
    }
}

#[tokio::main]
async fn main() -> Result<()>{
    let mut engine = scripts::init_engine().unwrap(); // script engine

    boot_log();

    let agent = "VIRTUE-C8DB26"; // TODO: agent token switching
   
    
    logger::log("I am now running your test functions...", logger::AlertType::DEFAULT, false);
    tests::run_tests(
        &statics::Lattice,
        &mut engine,
        &mut statics::Config.lock().unwrap(),
        &statics::Limiter
    ).await;

    // logger::log("Now executing scripts...", logger::AlertType::DEFAULT, false);
    // handle_scripts(&engine);
    
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