// This module contains the rate limiter implementation. It is based on the token bucket algorithm.
// The rate limiter is used to limit the number of requests per second to the SpaceTraders API.
// Compare this snippet from src\entities\schemas.rs:

use spacedust::apis::configuration::Configuration;
use spacedust::apis::*;
use spacedust::models::*;
use leaky_bucket_lite::LeakyBucket;
use std::time::Duration;
use std::future::Future;
use futures::executor::block_on;
use crate::statics::*;

// singleton rate limiter for whole program to send requests through (initialized in main)
pub struct RateLimiter{
    pub bucket: LeakyBucket
}

impl RateLimiter{
    /* 
    * Creates a new bucket
    * @param max: u32 - The maximum number of tokens the bucket can hold
    * @param interval_sec: u32 - The interval in seconds at which the bucket refills
    * @param amount: u32 - The amount of tokens the bucket refills at each interval
    * @return LeakyBucket - The rate limiter

    For our rate limiter we want:
        * 3 requests per second.
    
        max: 3
        interval_sec: 1
        amount: 3
    */
    pub fn new_bucket(max: u32, interval_sec: u64, amount: u32) -> LeakyBucket{
        LeakyBucket::builder()
            .max(max)
            .tokens(max)
            .refill_interval(Duration::from_secs(interval_sec))
            .refill_amount(amount)
            .build()
    }
}
/* 
    * Initializes the rate limiter
    * @param config: &Configuration - The API configuration
    * @return LeakyBucket - The rate limiter
*/
// called in main to initialize rate limiter for whole program to send requests through
pub fn init_rate_limiter(config: &Config) -> RateLimiter{
    let mut bucket = RateLimiter::new_bucket(2, 1, 2);
    RateLimiter{
        bucket: bucket
    }
}



/*
bucket.acquire_one().await; // wait for a token to be available. Blocks the next block from running until a token is available.
{
// This block is ran once the token is available
// We can make the request logic here in entities module
}
*/

/* 
    * Adds a request to the rate limiter
*/
// pub fn add_request(config: &Configuration, bucket: &mut LeakyBucket) -> Result<(), Box<dyn std::error::Error>>{
//     // Wait for a token to be available
//     bucket.acquire_one();
//     // Make the request
//     // TODO: generic request typing
//     //let response = get_my_user(&config);
//     // Return the result
//     unimplemented!("add_request() not implemented")
// }