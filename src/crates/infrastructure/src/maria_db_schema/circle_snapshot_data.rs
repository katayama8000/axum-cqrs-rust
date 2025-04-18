// -- スナップショットテーブル
// CREATE TABLE IF NOT EXISTS circle_snapshots (
//     id BIGINT AUTO_INCREMENT PRIMARY KEY,
//     circle_id CHAR(36) NOT NULL,
//     version INT NOT NULL,
//     state JSON NOT NULL,
//     created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
//     INDEX idx_circle_version (circle_id, version DESC)
// );

use std::str::FromStr;

use anyhow::Context;
use chrono::NaiveDateTime;
use domain::aggregate::{
    circle::Circle,
    value_object::{circle_id::CircleId, version::Version},
};
use sqlx::{types::Json, Row};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub(crate) struct CircleSnapshotData {
    /// スナップショットの一意識別子 (自動採番)
    pub id: i64,
    /// スナップショットの対象となるサークルの一意識別子 (UUID)
    pub circle_id: String,
    /// スナップショット作成時点での集約のバージョン番号
    pub version: i32,
    /// サークル集約の完全な状態をJSONとして格納
    pub state: Json<State>,
    /// スナップショットが作成された日時
    pub created_at: NaiveDateTime,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub(crate) struct State {
    id: String,
    name: String,
    capacity: i16,
    version: i32,
}

impl CircleSnapshotData {
    pub fn from_row(row: &sqlx::mysql::MySqlRow) -> Self {
        Self {
            id: row.get("id"),
            circle_id: row.get("circle_id"),
            version: row.get("version"),
            state: row.get::<Json<State>, _>("state"),
            created_at: row.get("created_at"),
        }
    }
}

impl State {
    pub fn from_circle(circle: &Circle) -> Result<Self, anyhow::Error> {
        let circle_id = circle.id.to_string();
        let name = circle.name.clone();
        let capacity = circle.capacity;
        let version = i32::try_from(circle.version)
            .map_err(|_| anyhow::Error::msg("Failed to convert version"))?;

        Ok(Self {
            id: circle_id,
            name,
            capacity,
            version,
        })
    }

    pub fn to_circle(&self) -> Result<Circle, anyhow::Error> {
        let circle_id = CircleId::from_str(&self.id).context("Failed to parse circle ID")?;
        let version = Version::try_from(self.version)
            .map_err(|_| anyhow::Error::msg("Failed to convert version"))?;
        let circle = Circle::restore_from_snapshot(
            circle_id,
            self.name.clone(),
            self.capacity,
            version.clone(),
        )
        .context("Failed to restore Circle from snapshot")?;

        Ok(circle)
    }
}

trait CircleExt {
    fn restore_from_snapshot(
        circle_id: CircleId,
        name: String,
        capacity: i16,
        version: Version,
    ) -> Result<Circle, anyhow::Error>;
}

impl CircleExt for Circle {
    fn restore_from_snapshot(
        circle_id: CircleId,
        name: String,
        capacity: i16,
        version: Version,
    ) -> Result<Circle, anyhow::Error> {
        Ok(Circle {
            id: circle_id,
            name,
            capacity,
            version,
        })
    }
}
