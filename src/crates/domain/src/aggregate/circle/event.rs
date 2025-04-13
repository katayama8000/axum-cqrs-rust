use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::aggregate::value_object::{circle_id::CircleId, event_id::EventId, version::Version};

#[derive(Clone, Debug)]
pub struct Event {
    pub circle_id: CircleId,
    pub data: EventData,
    pub id: EventId,
    pub occurred_at: NaiveDateTime,
    pub version: Version,
}

impl Event {
    pub fn build(circle_id: CircleId, version: Version) -> EventBuilder {
        EventBuilder {
            circle_id,
            id: EventId::gen(),
            occurred_at: Utc::now().naive_utc(),
            version,
        }
    }
}

pub struct EventBuilder {
    circle_id: CircleId,
    id: EventId,
    occurred_at: NaiveDateTime,
    version: Version,
}

impl EventBuilder {
    pub fn circle_created(self, name: String, capacity: i16) -> Event {
        Event {
            circle_id: self.circle_id,
            data: CircleCreated { name, capacity }.into(),
            id: self.id,
            occurred_at: self.occurred_at,
            version: self.version,
        }
    }

    pub fn circle_updated(self, name: Option<String>, capacity: Option<i16>) -> Event {
        Event {
            circle_id: self.circle_id,
            data: CircleUpdated { name, capacity }.into(),
            id: self.id,
            occurred_at: self.occurred_at,
            version: self.version,
        }
    }
}

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
