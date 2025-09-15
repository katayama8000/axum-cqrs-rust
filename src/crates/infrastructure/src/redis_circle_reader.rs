use std::str::FromStr;

use anyhow::Error;
use domain::{
    aggregate::{circle::Circle, value_object::circle_id::CircleId},
    interface::query::circle_reader_interface::CircleReaderInterface,
};
use redis::{AsyncCommands, Client};

#[derive(Clone, Debug)]
pub struct RedisCircleReader {
    client: Client,
}

impl RedisCircleReader {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    fn circle_key(&self, circle_id: &CircleId) -> String {
        format!("circle:{}", circle_id.to_string())
    }

    fn circles_list_key(&self) -> String {
        "circles:list".to_string()
    }
}

#[async_trait::async_trait]
impl CircleReaderInterface for RedisCircleReader {
    async fn get_circle(&self, circle_id: CircleId) -> Result<Option<Circle>, Error> {
        tracing::info!("find_circle_by_id from Redis: {:?}", circle_id);
        
        let mut conn = self.client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| anyhow::Error::msg(format!("Failed to connect to Redis: {}", e)))?;

        let key = self.circle_key(&circle_id);
        let json_data: Option<String> = conn
            .get(&key)
            .await
            .map_err(|e| anyhow::Error::msg(format!("Failed to get circle from Redis: {}", e)))?;

        match json_data {
            Some(data) => {
                let circle: Circle = serde_json::from_str(&data)
                    .map_err(|e| anyhow::Error::msg(format!("Failed to deserialize circle: {}", e)))?;
                Ok(Some(circle))
            }
            None => Ok(None),
        }
    }

    async fn list_circles(&self) -> Result<Vec<Circle>, Error> {
        tracing::info!("list_circles from Redis");
        
        let mut conn = self.client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| anyhow::Error::msg(format!("Failed to connect to Redis: {}", e)))?;

        let circle_ids: Vec<String> = conn
            .smembers(self.circles_list_key())
            .await
            .map_err(|e| anyhow::Error::msg(format!("Failed to get circle list from Redis: {}", e)))?;

        let mut circles = Vec::new();
        
        for circle_id_str in circle_ids {
            let circle_id = CircleId::from_str(&circle_id_str)
                .map_err(|e| anyhow::Error::msg(format!("Invalid circle ID: {}", e)))?;
            
            if let Some(circle) = self.get_circle(circle_id).await? {
                circles.push(circle);
            }
        }

        Ok(circles)
    }
}