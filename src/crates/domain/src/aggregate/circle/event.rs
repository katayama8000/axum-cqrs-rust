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
    // CircleCreated(CircleCreated),
    // CircleDeleted(CircleDeleted),
}
