// CREATE TABLE IF NOT EXISTS circle_events (
//     id CHAR(36) NOT NULL PRIMARY KEY,
//     circle_id CHAR(36) NOT NULL,
//     version INT NOT NULL,
//     event_type VARCHAR(100) NOT NULL,
//     payload JSON NOT NULL,
//     occurred_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
// );

use chrono::NaiveDateTime;
use sqlx::{types::Json, Row};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub(crate) struct CircleEventData {
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
