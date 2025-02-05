mod member_created;
mod member_deleted;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MemberEvent {
    MemberCreated(member_created::MemberCreated),
    MemberDeleted(member_deleted::MemberDeleted),
}
