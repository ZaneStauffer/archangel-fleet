// This module contains the rate limiter implementation. It is based on the token bucket algorithm.
// The rate limiter is used to limit the number of requests per second to the SpaceTraders API.
// Compare this snippet from src\entities\schemas.rs:

#[derive(Debug, Serialize, Deserialize)]
// "This is a bucket."
// "Dear god..."
// "There's more."
// "No..."
pub struct RateLimiter{
    pub max_tokens: u32,
    pub tokens: u32,
    pub last_update: u128,
    pub update_interval: u128
}
impl RateLimiter{
    pub fn new(max_tokens: u32, update_interval: u128) -> RateLimiter{
        RateLimiter{
            max_tokens: max_tokens,
            tokens: max_tokens,
            last_update: 0,
            update_interval: update_interval
        }
    }
    pub fn update(&mut self, current_time: u128){
        let time_passed = current_time - self.last_update;
        let tokens_to_add = time_passed / self.update_interval;
        self.tokens = self.tokens + tokens_to_add as u32;
        if self.tokens > self.max_tokens{
            self.tokens = self.max_tokens;
        }
        self.last_update = current_time;
    }
    pub fn consume(&mut self, tokens: u32) -> bool{
        if self.tokens >= tokens{
            self.tokens = self.tokens - tokens;
            true
        }else{
            false
        }
    }
}


