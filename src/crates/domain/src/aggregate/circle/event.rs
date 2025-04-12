use serde::{Deserialize, Serialize};

use crate::aggregate::value_object::{circle_id::CircleId, event_id::EventId, version::Version};

#[derive(Clone, Debug)]
pub struct Event {
    pub circle_id: CircleId,
    pub data: EventData,
    pub id: EventId,
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub version: Version,
}

impl Event {
    pub fn new<D>(
        circle_id: CircleId,
        data: D,
        id: EventId,
        occurred_at: chrono::DateTime<chrono::Utc>,
        version: Version,
    ) -> Self
    where
        D: Into<EventData>,
    {
        Self {
            data: data.into(),
            circle_id,
            id,
            version,
            occurred_at,
        }
    }
}

// this is a schema for command database
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum EventData {
    CircleCreated(CircleCreated),
    CircleUpdated(CircleUpdated),
}

impl From<CircleCreated> for EventData {
    fn from(created: CircleCreated) -> Self {
        Self::CircleCreated(created)
    }
}

impl From<CircleUpdated> for EventData {
    fn from(updated: CircleUpdated) -> Self {
        Self::CircleUpdated(updated)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CircleCreated {
    pub name: String,
    pub capacity: i16,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CircleUpdated {
    pub name: Option<String>,
    pub capacity: Option<i16>,
}
