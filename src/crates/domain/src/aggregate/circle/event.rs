use crate::aggregate::value_object::{circle_id::CircleId, event_id::EventId, version::Version};

#[derive(Clone, Debug)]
pub struct Event {
    pub data: EventData,
    pub circle_id: CircleId,
    pub id: EventId,
    pub version: Version,
}

impl Event {
    pub fn new<D>(data: D, circle_id: CircleId, id: EventId, version: Version) -> Self
    where
        D: Into<EventData>,
    {
        Self {
            data: data.into(),
            circle_id,
            id,
            version,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
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
