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
use std::sync::Arc;
use futures::executor::block_on;

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
        pub token: Option<String>,
        pub headquarters: Option<String>,
        pub credits: i32, // should start at 10000 credits
        pub starting_faction: String,
        pub account_id: Option<String>
        // pub rate limiter
        // pub config
    }
    impl Agents {
        // instantiates agent
        pub fn new(sym: String, faction: &str) -> Agents{
            Agents{
                symbol: sym,
                token: None,
                headquarters: None, // set at registration
                credits: 10000,
                starting_faction: faction.to_string(),
                account_id: None // set at registration
            }
        }
        // Converts primary key
        pub fn p_key(&self) -> Vec<u8>{
            self.symbol.as_bytes().to_vec()
        }
        // Attempts to register a new agent with the API
        pub async fn register(&mut self, config: &mut Configuration, limiter: &mut rate::RateLimiter,
            faction: Faction,
            symbol: String,
            tables: &mut Db) -> Register201Response
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
                        token: Some(res.data.token.clone()),
                        headquarters: Some(res.data.agent.headquarters.clone()),
                        credits: res.data.agent.credits,
                        account_id: Some(res.data.agent.account_id.clone()),
                        starting_faction: res.data.agent.starting_faction.clone()
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
        pub fn get_data(&self, config: &Configuration, limiter: &rate::RateLimiter) -> Agent {
            // acquire token from bucket
            block_on(limiter.bucket.acquire_one());
            let response = block_on(get_my_agent(config)).unwrap();
            let unwrapped = *response.data;
            unwrapped
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

    impl Clone for Agents{
        fn clone(&self) -> Self{
            Agents{
                symbol: self.symbol.clone(),
                token: self.token.clone(),
                headquarters: self.headquarters.clone(),
                credits: self.credits.clone(),
                starting_faction: self.starting_faction.clone(),
                account_id: self.account_id.clone()
            }
        }
    }