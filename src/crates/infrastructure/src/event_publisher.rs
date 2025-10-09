use anyhow::Result;
use domain::aggregate::circle::event::CircleEvent;
use tokio::sync::mpsc;
use redis::AsyncCommands;

#[async_trait::async_trait]
pub trait EventPublisher: Send + Sync + std::fmt::Debug {
    async fn publish(&self, events: Vec<CircleEvent>) -> Result<()>;
}

#[derive(Debug)]
pub struct InMemoryEventPublisher {
    sender: mpsc::UnboundedSender<CircleEvent>,
}

impl InMemoryEventPublisher {
    pub fn new() -> (Self, mpsc::UnboundedReceiver<CircleEvent>) {
        let (sender, receiver) = mpsc::unbounded_channel();
        (Self { sender }, receiver)
    }
}

#[async_trait::async_trait]
impl EventPublisher for InMemoryEventPublisher {
    async fn publish(&self, events: Vec<CircleEvent>) -> Result<()> {
        for event in events {
            self.sender.send(event)
                .map_err(|_| anyhow::Error::msg("Failed to send event"))?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct RedisProjectionHandler {
    redis_client: redis::Client,
    db: sqlx::MySqlPool,
}

impl RedisProjectionHandler {
    pub fn new(redis_client: redis::Client, db: sqlx::MySqlPool) -> Self {
        Self { redis_client, db }
    }

    pub async fn handle_event(&self, event: CircleEvent) -> Result<()> {
        tracing::info!("Handling event for Redis projection: {:?}", event.circle_id);
        
        let circle = self.rebuild_circle_from_events(&event.circle_id).await?;
        
        self.save_circle_to_redis(&circle).await?;
        
        Ok(())
    }

    async fn rebuild_circle_from_events(&self, circle_id: &domain::aggregate::value_object::circle_id::CircleId) -> Result<domain::aggregate::circle::Circle> {
        use crate::maria_db_schema::CircleEventData;
        use crate::circle_repository::EventExt;
        
        // fetch all events for the Circle ID from MySQL
        let query = sqlx::query(
            "SELECT * FROM circle_events WHERE circle_id = ? ORDER BY version ASC"
        ).bind(circle_id.to_string());
        
        let rows = query.fetch_all(&self.db).await
            .map_err(|e| anyhow::Error::msg(format!("Failed to fetch events: {}", e)))?;
        
        if rows.is_empty() {
            return Err(anyhow::Error::msg("No events found for circle"));
        }
        
        // Build event data
        let events = rows.iter()
            .map(|row| {
                let event_data = CircleEventData::from_row(row);
                CircleEvent::from_circle_event_data(event_data)
            })
            .collect::<Result<Vec<CircleEvent>, _>>()?;
        
        // Rebuild Circle
        Ok(domain::aggregate::circle::Circle::replay(events))
    }

    async fn save_circle_to_redis(&self, circle: &domain::aggregate::circle::Circle) -> Result<()> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await
            .map_err(|e| anyhow::Error::msg(format!("Failed to connect to Redis: {}", e)))?;
        
        let circle_id_str = circle.id.to_string();
        let circle_json = serde_json::to_string(circle)
            .map_err(|e| anyhow::Error::msg(format!("Failed to serialize circle: {}", e)))?;
        
        // Save Circle data
        let _: () = conn.set(format!("circle:{}", circle_id_str), circle_json).await
            .map_err(|e| anyhow::Error::msg(format!("Failed to save circle to Redis: {}", e)))?;
        
        // Add Circle ID to list
        let _: () = conn.sadd("circles:list", &circle_id_str).await
            .map_err(|e| anyhow::Error::msg(format!("Failed to add to circles list: {}", e)))?;
        
        tracing::info!("Successfully saved circle {} to Redis", circle_id_str);
        Ok(())
    }

    pub async fn start_processing(&self, mut receiver: mpsc::UnboundedReceiver<CircleEvent>) {
        while let Some(event) = receiver.recv().await {
            if let Err(e) = self.handle_event(event).await {
                tracing::error!("Failed to process event for Redis: {:?}", e);
                // Error handling: retry logic or dead-letter queue, etc.
            }
        }
    }
}