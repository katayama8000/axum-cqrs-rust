use std::str::FromStr;

use domain::{
    aggregate::{
        circle::{
            event::{self, Event},
            Circle,
        },
        value_object::{
            circle_id::CircleId,
            event_id::EventId,
            version::{self, Version},
        },
    },
    interface::command::circle_repository_interface::CircleRepositoryInterface,
};

use crate::maria_db_schema::{
    circle_event_data::CircleEventData, circle_protection_data::CircleProtectionData,
};

#[derive(Clone, Debug)]
pub struct CircleRepository {
    db: sqlx::MySqlPool,
}

impl CircleRepository {
    pub fn new(db: sqlx::MySqlPool) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl CircleRepositoryInterface for CircleRepository {
    async fn find_by_id(&self, circle_id: &CircleId) -> Result<Circle, anyhow::Error> {
        tracing::info!("find_circle_by_id : {:?}", circle_id);
        let event_query = sqlx::query("SELECT * FROM circle_events WHERE circle_id = ?")
            .bind(circle_id.to_string());
        let event_rows = event_query.fetch_all(&self.db).await.map_err(|e| {
            eprintln!("Failed to fetch circle events by circle_id: {:?}", e);
            anyhow::Error::msg("Failed to fetch circle events by circle_id")
        })?;

        let event_data = event_rows
            .iter()
            .map(|row| {
                CircleEventData::try_from_row(row)
                    .and_then(|data| Event::from_circle_event_data(data))
            })
            .collect::<Result<Vec<Event>, _>>()?;

        // Sort events by version
        let mut event_data = event_data;
        event_data.sort_by(|a, b| a.version.cmp(&b.version));
        Ok(Circle::from_events(event_data.clone()))
    }

    async fn store(
        &self,
        _version: Option<version::Version>,
        events: Vec<event::Event>,
    ) -> Result<(), anyhow::Error> {
        if events.is_empty() {
            tracing::info!("No events to store");
            return Ok(());
        }

        let events_for_logging = events.clone();

        // First transaction for storing events
        {
            let mut transaction = self.db.begin().await?;

            // Store events
            for event in events {
                let event_data = CircleEventData::try_from(event.clone())?;

                sqlx::query("INSERT INTO circle_events (circle_id, id, occurred_at, event_type, version, payload) VALUES (?, ?, ?, ?, ?, ?)")
                .bind(event_data.circle_id.clone())
                .bind(event_data.id)
                .bind(event_data.occurred_at)
                .bind(event_data.event_type.clone())
                .bind(event_data.version)
                .bind(event_data.payload.clone())
                .execute(&mut *transaction)
                .await.map_err(|e| {
                    eprintln!("Failed to insert circle event: {:?}", e);
                    anyhow::Error::msg("Failed to insert circle event")
                })?;
            }

            transaction.commit().await?;
        }

        // Second transaction for updating projections
        {
            let mut transaction = self.db.begin().await?;

            // Update projection in a separate transaction
            let mut current_circle = self.find_by_id(&events_for_logging[0].circle_id).await?;
            // 新しいイベントを適用
            for event in &events_for_logging {
                current_circle.apply_event(event);
            }
            let data = CircleProtectionData::from(current_circle.clone());

            sqlx::query("REPLACE INTO circle_projections (circle_id, name, capacity, version) VALUES (?, ?, ?, ?)",)
                .bind(data.id.to_string())
                .bind(data.name.to_string())
                .bind(data.capacity)
                .bind(data.version)
                .execute(&mut *transaction)
                .await.map_err(|e| {
                    eprintln!("Failed to update circle projection: {:?}", e);
                    anyhow::Error::msg("Failed to update circle projection")
                })?;

            transaction.commit().await?;
        }

        tracing::info!("Stored circle events: {:?}", events_for_logging);
        Ok(())
    }
}

trait EventExt {
    fn from_circle_event_data(event_data: CircleEventData) -> Result<Self, anyhow::Error>
    where
        Self: Sized;
}

impl EventExt for Event {
    fn from_circle_event_data(v: CircleEventData) -> Result<Self, anyhow::Error> {
        let event: event::EventData = serde_json::from_str(&v.payload.to_string())?;
        Ok(Self {
            id: EventId::from_str(&v.id)?,
            circle_id: CircleId::from_str(&v.circle_id)?,
            version: Version::try_from(v.version)
                .map_err(|_| anyhow::Error::msg("Failed to convert version from string"))?,
            data: event,
            occurred_at: v.occurred_at,
        })
    }
}

// event -> circle_event_data
impl TryFrom<event::Event> for CircleEventData {
    type Error = anyhow::Error;
    fn try_from(value: event::Event) -> Result<Self, Self::Error> {
        let event_type = match value.data.clone() {
            event::EventData::CircleCreated(_) => "circle_created",
            event::EventData::CircleUpdated(_) => "circle_updated",
        };

        let event_data = CircleEventData {
            circle_id: value.circle_id.to_string(),
            event_type: event_type.to_string(),
            id: value.id.to_string(),
            occurred_at: value.occurred_at,
            payload: sqlx::types::Json(serde_json::to_value(value.data)?),
            version: value
                .version
                .try_into()
                .map_err(|_| anyhow::Error::msg("Failed to convert version to i32"))?,
        };
        Ok(event_data)
    }
}

#[cfg(test)]
mod tests {}
