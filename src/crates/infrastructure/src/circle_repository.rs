use std::str::FromStr;

use domain::{
    aggregate::{
        circle::{
            event::{self, CircleEvent},
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
    circle_snapshot_data::State, CircleEventData, CircleSnapshotData,
};

const SNAPSHOT_INTERVAL: i32 = 5;

#[derive(Clone, Debug)]
pub struct CircleRepository {
    db: sqlx::MySqlPool,
}

impl CircleRepository {
    pub fn new(db: sqlx::MySqlPool) -> Self {
        Self { db }
    }

    async fn get_latest_snapshot(
        &self,
        circle_id: &CircleId,
    ) -> Result<Option<(Circle, Version)>, anyhow::Error> {
        let query = sqlx::query(
            "SELECT * FROM circle_snapshots WHERE circle_id = ? ORDER BY version DESC LIMIT 1",
        )
        .bind(circle_id.to_string());

        let row = match query.fetch_optional(&self.db).await {
            Ok(Some(row)) => row,
            Ok(None) => return Ok(None),
            Err(e) => {
                tracing::error!("Failed to fetch snapshot: {:?}", e);
                return Err(anyhow::Error::msg("Failed to fetch circle snapshot"));
            }
        };

        let snapshot = CircleSnapshotData::from_row(&row);
        let circle = snapshot.state.to_circle()?;
        let version = Version::try_from(snapshot.version)
            .map_err(|_| anyhow::Error::msg("Failed to convert version from i32"))?;

        Ok(Some((circle, version)))
    }

    async fn save_snapshot(&self, circle: &Circle) -> Result<(), anyhow::Error> {
        let circle_id = circle.id.to_string();
        let version: i32 = circle.version.try_into().map_err(|_| {
            tracing::error!("Failed to convert version to i32");
            anyhow::Error::msg("Failed to convert version to i32")
        })?;
        let state = State::from_circle(circle).map_err(|e| {
            tracing::error!("Failed to convert circle to state: {:?}", e);
            anyhow::Error::msg("Failed to convert circle to state")
        })?;

        sqlx::query(
            "INSERT INTO circle_snapshots (circle_id, version, state) 
             VALUES (?, ?, ?)",
        )
        .bind(&circle_id)
        .bind(version)
        .bind(&sqlx::types::Json(state)) // Json型に明示的に変換
        .execute(&self.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to save snapshot: {:?}", e);
            anyhow::Error::msg("Failed to save circle snapshot")
        })?;

        tracing::info!(
            "Saved snapshot for circle {} at version {}",
            circle_id,
            version
        );
        Ok(())
    }
}

#[async_trait::async_trait]
impl CircleRepositoryInterface for CircleRepository {
    async fn find_by_id(&self, circle_id: &CircleId) -> Result<Circle, anyhow::Error> {
        tracing::info!("find_circle_by_id : {:?}", circle_id);

        // check snapshot
        if let Ok(Some((mut circle, snapshot_version))) = self.get_latest_snapshot(circle_id).await
        {
            tracing::info!(
                "Found snapshot for circle {:?} at version {:?}",
                circle_id,
                snapshot_version
            );

            let version_i32: i32 = snapshot_version.try_into().map_err(|_| {
                tracing::error!("Failed to convert version to i32");
                anyhow::Error::msg("Failed to convert version to i32")
            })?;
            let event_query = sqlx::query(
                "SELECT * FROM circle_events WHERE circle_id = ? AND version > ? ORDER BY version ASC"
            )
            .bind(circle_id.to_string())
            .bind(version_i32);

            let event_rows = event_query.fetch_all(&self.db).await.map_err(|e| {
                tracing::error!("Failed to fetch circle events after snapshot: {:?}", e);
                anyhow::Error::msg("Failed to fetch circle events after snapshot")
            })?;

            println!("event_rows: {:?}", event_rows);

            if !event_rows.is_empty() {
                let events = event_rows
                    .iter()
                    .map(|row| CircleEvent::from_circle_event_data(CircleEventData::from_row(row)))
                    .collect::<Result<Vec<CircleEvent>, _>>()?;

                for event in events {
                    circle.apply_event(&event);
                }
            }

            return Ok(circle);
        }

        let event_query =
            sqlx::query("SELECT * FROM circle_events WHERE circle_id = ? ORDER BY version ASC")
                .bind(circle_id.to_string());
        let event_rows = event_query.fetch_all(&self.db).await.map_err(|e| {
            eprintln!("Failed to fetch circle events by circle_id: {:?}", e);
            anyhow::Error::msg("Failed to fetch circle events by circle_id")
        })?;

        if event_rows.is_empty() {
            return Err(anyhow::Error::msg("Circle not found"));
        }

        let event_data = event_rows
            .iter()
            .map(|row| CircleEvent::from_circle_event_data(CircleEventData::from_row(row)))
            .collect::<Result<Vec<CircleEvent>, _>>()?;

        let mut event_data = event_data;
        event_data.sort_by(|a, b| a.version.cmp(&b.version));
        let circle = Circle::replay(event_data);

        Ok(circle)
    }

    async fn store(
        &self,
        _version: Option<version::Version>,
        events: Vec<event::CircleEvent>,
    ) -> Result<(), anyhow::Error> {
        if events.is_empty() {
            tracing::info!("No events to store");
            return Ok(());
        }

        let events_for_logging = events.clone();

        // First transaction for storing events
        {
            let mut transaction = self.db.begin().await?;

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

        let first_event = events_for_logging
            .first()
            .ok_or_else(|| anyhow::Error::msg("No events found"))?;
        let mut current_circle = self.find_by_id(&first_event.circle_id).await?;

        for event in &events_for_logging {
            current_circle.apply_event(event);
        }

        // Save snapshot if needed
        let version_i32: i32 = current_circle.version.try_into().map_err(|e| {
            anyhow::Error::msg(format!("Failed to convert version to i32: {:?}", e))
        })?;
        if version_i32 % SNAPSHOT_INTERVAL == 0 {
            if let Err(e) = self.save_snapshot(&current_circle).await {
                tracing::error!("Failed to save snapshot: {:?}", e);
            } else {
                tracing::info!("Snapshot saved for circle at version {}", version_i32);
            }
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

impl EventExt for CircleEvent {
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
impl TryFrom<event::CircleEvent> for CircleEventData {
    type Error = anyhow::Error;
    fn try_from(value: event::CircleEvent) -> Result<Self, Self::Error> {
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
