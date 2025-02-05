use crate::aggregate::value_object::{circle_id::CircleId, event_id::EventId, version::Version};

#[derive(Clone, Debug)]
pub(crate) struct Event {
    pub data: EventData,
    pub member_id: CircleId,
    pub id: EventId,
    pub version: Version,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventData {
    MemberCreated(MemberCreated),
    MemberDeleted(MemberDeleted),
}

impl From<MemberCreated> for EventData {
    fn from(created: MemberCreated) -> Self {
        Self::MemberCreated(created)
    }
}

impl From<MemberDeleted> for EventData {
    fn from(deleted: MemberDeleted) -> Self {
        Self::MemberDeleted(deleted)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct MemberCreated {
    pub member_id: String,
    pub name: String,
    pub age: i16,
    pub grade: String,
    pub major: String,
    pub version: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct MemberDeleted {
    pub member_id: String,
    pub version: i64,
}
