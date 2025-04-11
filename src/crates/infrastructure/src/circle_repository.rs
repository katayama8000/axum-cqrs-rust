use std::str::FromStr;

use chrono;
use domain::{
    aggregate::{
        circle::{
            event::{self, Event},
            Circle,
        },
        value_object::{circle_id::CircleId, event_id::EventId, version},
    },
    interface::command::circle_repository_interface::CircleRepositoryInterface,
};
use sqlx::Row;

use crate::maria_db_schema::circle_event_data::CircleEventData;

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
                let circle_event_data = CircleEventData::from_row(row);
                Event::from_circle_event_data(circle_event_data)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Circle::from_events(event_data.clone()))
    }

    async fn store(
        &self,
        version: Option<version::Version>,
        events: Vec<event::Event>,
    ) -> Result<(), anyhow::Error> {
        if events.is_empty() {
            return Ok(());
        }

        let mut transaction = self.db.begin().await?;

        // Optimistic concurrency control using version
        if let Some(expected_version) = version {
            let circle_id = &events[0].circle_id;

            // Check if the current version matches the expected version
            let version_query = sqlx::query(
                "SELECT MAX(version) as current_version FROM circle_events WHERE circle_id = ?",
            )
            .bind(circle_id.to_string());

            let version_row = version_query.fetch_one(&mut *transaction).await?;
            let current_version: Option<u32> = version_row.try_get("current_version")?;

            // Convert database version to domain version
            let current_version = match current_version {
                Some(v) => version::Version::from(v),
                None => version::Version::from(0), // Initial version if no events exist
            };

            // Version conflict check
            if current_version != expected_version {
                return Err(anyhow::Error::msg(format!(
                    "Concurrency conflict: expected version {}, but current version is {}",
                    expected_version, current_version
                )));
            }
        }

        let events_for_logging = events.clone();
        for event in events {
            let event_data = CircleEventData::try_from(event.clone())?;
            sqlx::query("INSERT INTO circle_events (circle_id, id, occurred_at, version, payload) VALUES (?, ?, ?, ?, ?)")
                .bind(event_data.circle_id)
                .bind(event_data.id)
                .bind(event_data.occurred_at)
                .bind(event_data.version)
                .bind(event_data.payload)
                .execute(&mut *transaction)
                .await?;
        }

        transaction.commit().await?;
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
    fn from_circle_event_data(event_data: CircleEventData) -> Result<Self, anyhow::Error> {
        let event: event::EventData = serde_json::from_str(&event_data.payload)?;
        Ok(Self {
            id: EventId::from_str(&event_data.id)?,
            circle_id: CircleId::from_str(&event_data.circle_id)?,
            version: event_data.version.into(),
            data: event,
            occurred_at: chrono::DateTime::parse_from_rfc3339(&event_data.occurred_at)?
                .with_timezone(&chrono::Utc),
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
            occurred_at: value.occurred_at.to_rfc3339(),
            payload: serde_json::to_string(&value.data)?,
            version: value.version.into(),
        };
        Ok(event_data)
    }
}

#[cfg(test)]
mod tests {}
