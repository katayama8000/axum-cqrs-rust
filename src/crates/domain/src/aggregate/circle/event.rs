use crate::aggregate::value_object::{circle_id::CircleId, event_id::EventId, version::Version};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Event {
    pub at: chrono::DateTime<chrono::Utc>,
    pub data: EventData,
    pub circle_id: CircleId,
    pub id: EventId,
    pub version: Version,
}

impl Event {
    pub fn new<D>(circle_id: CircleId, version: Version, data: D) -> Event
    where
        D: Into<EventData>,
    {
        Self {
            at: chrono::Utc::now(),
            data: data.into(),
            circle_id,
            id: EventId::generate(),
            version,
        }
    }
}

impl From<CircleCreated> for EventData {
    fn from(data: CircleCreated) -> Self {
        Self::CircleCreated(data)
    }
}

impl From<CircleUpdated> for EventData {
    fn from(data: CircleUpdated) -> Self {
        Self::CircleUpdated(data)
    }
}

impl From<CircleDeleted> for EventData {
    fn from(data: CircleDeleted) -> Self {
        Self::CircleDeleted(data)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventData {
    CircleCreated(CircleCreated),
    CircleUpdated(CircleUpdated),
    CircleDeleted(CircleDeleted),
}

// TODO: Rethink about the design of the event data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CircleCreated {
    pub name: String,
    pub owner_id: String,
    pub capacity: i16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CircleUpdated {
    pub name: Option<String>,
    pub capacity: Option<i16>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CircleDeleted {
    pub circle_id: String,
}
