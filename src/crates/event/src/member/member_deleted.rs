#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct MemberDeleted {
    pub member_id: String,
    pub version: i64,
}
