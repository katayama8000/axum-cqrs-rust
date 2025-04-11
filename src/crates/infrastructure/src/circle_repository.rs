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
        event: &Event,
    ) -> Result<(), anyhow::Error> {
        todo!("store");
        // sqlx::query(
        //     r#"
        //     INSERT INTO circle_events (
        //         id,
        //         circle_id,
        //         version,
        //         payload,
        //         occurred_at
        //     ) VALUES (?, ?, ?, ?, ?, ?)
        //     "#,
        // )
        // .bind(event.id.to_string())
        // .bind(event.circle_id.to_string())
        // .bind(event.version.to_string())
        // .bind(serde_json::to_value(&event.data)?) // イベント本体
        // .bind(event.occurred_at.to_string())
        // .execute(&self.db)
        // .await?;
        // Ok(())
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

#[cfg(test)]
mod tests {}
