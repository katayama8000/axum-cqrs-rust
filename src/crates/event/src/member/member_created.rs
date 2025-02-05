#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct MemberCreated {
    pub member_id: String,
    pub name: String,
    pub age: i16,
    pub grade: i16,
    pub major: String,
    pub version: i64,
}
