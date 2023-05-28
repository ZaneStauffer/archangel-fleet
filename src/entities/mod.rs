pub mod schemas{
    use spacedust::apis::agents_api::get_my_agent;
    use spacedust::models::get_my_agent_200_response::GetMyAgent200Response; 
    use spacedust::apis::agents_api::GetMyAgentError;
    use spacedust::models::agent::Agent;
    use spacedust::apis::configuration::Configuration;
    use spacedust::apis::default_api::register;
    use spacedust::models::register_request::{Faction, RegisterRequest};
    use spacedust::models::register_201_response::Register201Response;
    use serde::{Deserialize, Serialize};
    use struct_db::*;
    use std::any::Any;
    use async_trait::async_trait;
    use std::result::Result;
    use std::pin::Pin;
    use std::future::Future;
    use crate::db;

    use crate::logger::Error;

    #[async_trait]
    pub trait Data{
        async fn get_data<R: 'static + Send, E: 'static + std::error::Error>(&self,
            config: &mut Configuration,
            tables: &Db
        ) -> Result<Box<dyn Any + Send>, E>;
    }

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
        pub async fn register(&self, config: &mut Configuration, faction: Faction, symbol: String, tables: &Db) -> Register201Response{
            let register_request = RegisterRequest::new(faction, symbol.clone());
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
    }

    #[async_trait]
    impl Data for Agents {
        async fn get_data<R: 'static + Send, E: 'static + std::error::Error>(
            &self,
            config: &mut Configuration,
            tables: &Db)  -> Result<Box<dyn Any + Send>, E> {
                let get = get_my_agent(config).await;
                Ok(Box::new(get.unwrap().data) as Box<dyn Any + Send>)
            }
    }

}