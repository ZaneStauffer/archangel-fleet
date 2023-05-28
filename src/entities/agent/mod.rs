use spacedust::apis::agents_api::get_my_agent;
use spacedust::apis::agents_api::GetMyAgentError;
use spacedust::models::agent::Agent;
use spacedust::apis::configuration::Configuration;
use spacedust::apis::default_api::register;
use spacedust::models::register_request::{Faction, RegisterRequest};
use spacedust::models::register_201_response::Register201Response;
use spacedust::models::get_status_200_response::GetStatus200Response;
use spacedust::apis::default_api::GetStatusError;
use serde::{Deserialize, Serialize};
use struct_db::*;
use std::result::Result;
use leaky_bucket_lite::LeakyBucket;

use crate::db;
use crate::rate;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
    #[struct_db(
        fn_primary_key(p_key),  // required
        //fn_secondary_key(s_key),  // optional
        // ... other fn_secondary_key ...
    )]
    pub struct Agents{
        pub symbol: String,
        pub token: String
    }
    impl Agents {
        // instantiates agent
        pub fn new(sym: String, token: String) -> Agents{
            Agents{
                symbol: sym,
                token: token
            }
        }
        // Converts primary key
        pub fn p_key(&self) -> Vec<u8>{
            self.symbol.as_bytes().to_vec()
        }
        // Attempts to register a new agent with the API
        // TODO: Buffer requests with the leaky bucket module to prevent rate limiting
        // In the bucket we should have the function return a promise for THIS function to handle. The promise
        // should be a promise to return a Register201Response. The bucket should be a singleton.
        // bucket has max 5 tokens. 3 tokens refill per second. 1 token per request.
        pub async fn register(&self, config: &mut Configuration, limiter: &rate::RateLimiter,
            faction: Faction,
            symbol: String,
            tables: &Db) -> Register201Response
        {
            let register_request = RegisterRequest::new(faction, symbol.clone());
            limiter.bucket.acquire_one().await;
            let register_response = register(&config, Some(register_request)).await;
            match register_response {
                Ok(res) => {
                    println!("{:#?}", res);
                    // update DB here
                    db::insert(tables, Agents{
                        symbol: symbol,
                        token: res.data.token.clone()
                    }).unwrap();
                    // Change access token for API (TODO: handle multiple tokens)
                    config.bearer_access_token = Some(res.data.token.clone());
                    res
                }
                Err(e) => {
                    // TODO: don't panic.
                    panic!("{:#?}", e);
                }
            }
        }
        /*
        Agent {
            account_id: "cli6nh1cu4pios60d4hvxfrqp",
            symbol: "VIRTUE-C8DB26",
            headquarters: "X1-NU19-03110X",
            credits: 100000,
            starting_faction: "QUANTUM",
        }
        */
        // Gets agent data from API
        pub async fn get_data(&self, config: &mut Configuration, limiter: &rate::RateLimiter) -> Result<Agent, GetMyAgentError> {
            // acquire token from bucket
            limiter.bucket.acquire_one().await;
            let response = get_my_agent(config).await.unwrap();
            println!("Token found!");
            Ok(*response.data)
        }
        /*
        GetStatus200Response {
            status: "SpaceTraders is currently online and available to play",
            version: "v2",
            reset_date: "2023-05-20",
            description: "SpaceTraders is a headless API and fleet-management game where players can work together or against each other to trade, explore, expand, and conquer in a dynamic and growing universe. Build your own UI, write automated scripts, or just play the game from the comfort of your terminal. The game is currently in alpha and is under active development.",
            stats: GetStatus200ResponseStats {
                agents: 2621,
                ships: 4359,
                systems: 8999,
                waypoints: 49978,
            },
            leaderboards: GetStatus200ResponseLeaderboards {
                etc...
        */
        pub async fn get_server_status(config: &mut Configuration, limiter: &rate::RateLimiter) -> Result<GetStatus200Response, GetStatusError>{
            limiter.bucket.acquire_one().await;
            let response = spacedust::apis::default_api::get_status(&config).await.unwrap();
            Ok(response)
        }
        

    }