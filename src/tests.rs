use spacedust::apis::agents_api::get_my_agent;
use spacedust::apis::configuration::Configuration;
use spacedust::apis::default_api::register;
use spacedust::models::register_request::{Faction, RegisterRequest};
use struct_db::*;
use colored::*;
use std::env;
use rhai::{Engine, EvalAltResult, AST};

use crate::rate::RateLimiter;
use crate::entities::schemas;
use crate::entities::{agent::*, ship::*};
use crate::database::db;
use crate::generators;
use crate::logger;
use crate::statics::*;

pub async fn run_tests(
    db: &Db,
    engine: &mut Engine,
    config: &mut Configuration,
    limiter: &RateLimiter
){
    _register_test(db, config, limiter).await;
}

// this will actually register an agent with SpaceTraders
// this tests the register function of Agents
async fn _register_test(_db: &Db, config: &mut Configuration, limiter: &RateLimiter){
    // create a new agent
    let sym = format!("VIRTUE-{}",generators::generate_hex_ID());
    let mut _test_agent = Agents::new(sym.clone());
    println!("test agent: {:#?}", _test_agent);
    _test_agent.register(config, limiter, "quantum", _db).await;
}

// this will insert a faux agent into the database, then read it back out. This will not actually register an agent with SpaceTraders.
async fn _db_test(_db: &Db, config: &Configuration){
    let sym = format!("VIRTUE-{}",generators::generate_hex_ID());
    logger::log("inserting test agent. (these agent structs will not be valid)", logger::AlertType::DEFAULT, false);
    db::insert(_db, Agents::new(sym.clone()));
    logger::log("test agent inserted. Now reading test agent:", logger::AlertType::DEFAULT, false);
    let _agent: Agents = db::read(_db, sym.clone());
    logger::log(format!("{:#?}", _agent).as_str(), logger::AlertType::DEFAULT, false);
}

async fn _rate_limit_test(config: &mut Configuration, limiter: &mut RateLimiter){
    // test rate limit by sending a lot of data
    let mut search = Agents::new("VIRTUE-C8DB26".to_string());
    for count in 0..100{
        let results = search.get_data(config, limiter);
        println!("{}: {:#?}",count, results);
    }
}

async fn _get_bearer_token_test(db: &Db, config: &mut Configuration, limiter: &mut RateLimiter){
    // register request [testing]
    let id: String = generators::generate_hex_ID();
    let user_agent = format!("VIRTUE-{}", id); // agent name with ARCHANGEL + id

    
}