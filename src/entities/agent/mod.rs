use spacedust::apis::agents_api::get_my_agent;
use spacedust::apis::factions_api::*;
use spacedust::apis::agents_api::GetMyAgentError;
use spacedust::models::agent::Agent;
use spacedust::models::*;
use spacedust::apis::configuration::Configuration;
use spacedust::apis::default_api::register;
use spacedust::models::register_request::{Faction, RegisterRequest};
use spacedust::models::register_201_response::Register201Response;
use spacedust::models::get_status_200_response::GetStatus200Response;
use spacedust::apis::default_api::GetStatusError;
use serde::{Deserialize, Serialize};
use struct_db::*;
use std::result::Result;
use futures::executor::block_on;

use crate::db;
use crate::rate;
use crate::logger;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
    #[struct_db(
        fn_primary_key(p_key),  // required
        //fn_secondary_key(s_key),  // optional
        // ... other fn_secondary_key ...
    )]
    pub struct Agents{
        pub symbol: String,
        pub token: String,
        pub headquarters: String,
        pub credits: i32, // should start at 10000 credits
        pub starting_faction: Option<faction::Faction>,
        pub account_id: String
    }

    impl Agents {
        // instantiates agent
        pub fn new(sym: String) -> Agents{
            Agents{
                symbol: sym,
                token: String::new(),
                headquarters: String::new(), // set at registration
                credits: 10000, // TODO: set at registration
                starting_faction: None,
                account_id: String::new() // set at registration
            }
        }
        // Converts primary key
        pub fn p_key(&self) -> Vec<u8>{
            self.symbol.as_bytes().to_vec()
        }

        // converts a spacedust api faction struct to a request enum faction
        fn faction_to_enum(&self, response_faction: faction::Faction) -> Result<Faction, logger::Error>{
            let _formatted = response_faction.symbol.to_uppercase();
            match _formatted.as_str(){
                "COSMIC" => Ok(Faction::Cosmic),
                "VOID" => Ok(Faction::Void),
                "GALACTIC" => Ok(Faction::Galactic),
                "QUANTUM" => Ok(Faction::Quantum),
                "DOMINION" => Ok(Faction::Dominion),
                _ => {
                    Err(logger::Error::APIError(String::from("Invalid faction symbol")))
                }
            }
        }
        // Attempts to register a new agent with the API
        pub async fn register(&mut self,
            config: &mut Configuration,
            limiter: & rate::RateLimiter,
            faction_symbol: &str,
            tables: & Db) -> Register201Response
        {
            match self.starting_faction {
                // if we already have a full faction, we dont need to ask the server
                Some(_) => {

                },
                // if we don't have a faction, we need to make a request to Space Traders API
                None => {
                    let _faction_res = get_faction(
                        config,
                        faction_symbol.to_uppercase().as_str()
                    ).await.unwrap();
                    let _faction = *_faction_res.data;
                    self.starting_faction = Some(_faction);
                }
            }

            // convert the response faction model to a request faction enum
            let _fac_enum = self.faction_to_enum(self.starting_faction.clone().unwrap()).unwrap();
            // Form the register request
            println!("faction: {:#?}", _fac_enum);
            let register_request = RegisterRequest::new(
                // the register request takes an enum, so we convert it
                _fac_enum,
                self.symbol.clone()
            );
            limiter.bucket.acquire_one().await;
            let register_response = register(&config, Some(register_request)).await;
            match register_response {
                Ok(res) => {
                    println!("{:#?}", res);
                    let _ref = *res.data.faction.clone();

                    db::insert(tables, Agents{
                        symbol: res.data.agent.symbol.clone(),
                        token: res.data.token.clone(),
                        headquarters: res.data.agent.headquarters.clone(),
                        credits: res.data.agent.credits,
                        account_id: res.data.agent.account_id.clone(),
                        starting_faction: Some(_ref)
                    }).unwrap();
                    
                    
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