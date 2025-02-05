#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CircleDeleted {
    pub circle_id: String,
    pub version: i64,
}
