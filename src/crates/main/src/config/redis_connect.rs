use dotenv::dotenv;
use redis::Client;
use std::env;

#[derive(Debug, Clone)]
struct RedisConfig {
    redis_url: String,
}

impl RedisConfig {
    fn from_env() -> Self {
        dotenv().ok();
        Self {
            redis_url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6380".to_string()),
        }
    }
}

pub fn connect() -> Result<Client, redis::RedisError> {
    let config = RedisConfig::from_env();
    println!("Connecting to Redis: {}", config.redis_url);
    let client = Client::open(config.redis_url)?;
    Ok(client)
}

#[cfg(test)]
pub fn connect_test() -> Result<Client, redis::RedisError> {
    // TODO: build a Redis connection for testing
    let config = RedisConfig::from_env();
    let client = Client::open(config.redis_url)?;
    Ok(client)
}