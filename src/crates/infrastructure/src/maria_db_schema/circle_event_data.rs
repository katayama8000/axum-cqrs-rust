use chrono::NaiveDateTime;
use sqlx::{types::Json, Row};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CircleEventData {
    pub id: String,
    pub circle_id: String,
    pub version: i32,
    pub event_type: String,
    pub payload: Json<serde_json::Value>,
    pub occurred_at: NaiveDateTime,
}

impl CircleEventData {
    pub fn from_row(row: &sqlx::mysql::MySqlRow) -> Self {
        Self {
            id: row.get("id"),
            circle_id: row.get("circle_id"),
            version: row.get("version"),
            event_type: row.get("event_type"),
            payload: row.get("payload"),
            occurred_at: row.get("occurred_at"),
        }
    }
}
