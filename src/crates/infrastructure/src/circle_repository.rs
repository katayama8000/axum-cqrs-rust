use std::str::FromStr;

use chrono::{self, NaiveDateTime};
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
            .map(|row| Event::from_circle_event_data(CircleEventData::from_row(row)))
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

        //     let event_type = match event.data {
        //         event::EventData::CircleCreated(_) => "circle_created",
        //         event::EventData::CircleUpdated(_) => "circle_updated",
        //     };

        //     let event_data = CircleEventData::try_from(event.clone())?;

        //     sqlx::query(
        //     "INSERT INTO circle_events (circle_id, id, occurred_at, event_type, version, payload) VALUES (?, ?, ?, ?, ?, ?)",
        // )
        // .bind(event_data.circle_id)
        // .bind(event_data.id)
        // .bind(event_data.occurred_at)
        // .bind(event_type)
        // .bind(event_data.version)
        // .bind(event_data.payload)
        // .execute(&self.db)
        // .await.map_err(
        //     |e| {
        //         eprintln!("Failed to store circle event: {:?}", e);
        //         anyhow::Error::msg("Failed to store circle event")
        //     },
        // )?;
        // }

        // tracing::info!("Stored circle events: {:?}", events_for_logging);

        let mut transaction = self.db.begin().await?;

        let events_for_logging = events.clone();
        for event in events {
            let event_data = CircleEventData::try_from(event.clone())?;
            sqlx::query("INSERT INTO circle_events (circle_id, id, occurred_at, event_type, version, payload) VALUES (?, ?, ?, ?, ?, ?)")
                .bind(event_data.circle_id)
                .bind(event_data.id)
                .bind(event_data.occurred_at)
                .bind(event_data.event_type)
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
        let occurred_at_naive: NaiveDateTime = value.occurred_at.naive_utc();

        let event_data = CircleEventData {
            circle_id: value.circle_id.to_string(),
            event_type: event_type.to_string(),
            id: value.id.to_string(),
            occurred_at: occurred_at_naive.to_string(),
            payload: serde_json::to_string(&value.data)?,
            version: value.version.into(),
        };
        Ok(event_data)
    }
}

#[cfg(test)]
mod tests {}
