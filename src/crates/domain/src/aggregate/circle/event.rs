use crate::aggregate::value_object::{circle_id::CircleId, event_id::EventId, version::Version};

#[derive(Clone, Debug)]
pub(crate) struct Event {
    pub data: EventData,
    pub circle_id: CircleId,
    pub id: EventId,
    pub version: Version,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventData {
    CircleCreated(CircleCreated),
    CircleDeleted(CircleDeleted),
}

impl From<CircleCreated> for EventData {
    fn from(created: CircleCreated) -> Self {
        Self::CircleCreated(created)
    }
}

impl From<CircleDeleted> for EventData {
    fn from(deleted: CircleDeleted) -> Self {
        Self::CircleDeleted(deleted)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CircleCreated {
    pub circle_id: String,
    pub name: String,
    pub capacity: i16,
    pub version: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CircleDeleted {
    pub circle_id: String,
    pub version: i64,
}
