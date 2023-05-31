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

pub async fn run_tests(
    db: &mut Db,
    engine: &mut Engine,
    config: &mut Configuration,
    limiter: &mut RateLimiter
){
    // everytime we want to test, we write a test function and call it here with the dependencies we inject
    // _rate_limit_test(config, limiter).await;
    // _get_bearer_token_test(db, config, limiter).await;
}
// for future tests please use assert!() and assert_eq!() macros to test the results of the function

async fn _rate_limit_test(config: &mut Configuration, limiter: &mut RateLimiter){
    // test rate limit by sending a lot of data
    let mut search = Agents::new("VIRTUE-C8DB26".to_string(), "".to_string());
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