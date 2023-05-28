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