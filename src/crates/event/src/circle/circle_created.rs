#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CircleCreated {
    pub circle_id: String,
    pub name: String,
    pub capacity: i16,
    pub version: i64,
}
