use std::str::FromStr;

use domain::aggregate::{
    circle::Circle,
    value_object::{circle_id::CircleId, version::Version},
};
use sqlx::Row;

// -- プロジェクション: circle_projections テーブルの作成
// CREATE TABLE IF NOT EXISTS circle_projections (
//     circle_id CHAR(36) NOT NULL PRIMARY KEY,
//     -- 集約ID（Circle ID）
//     name VARCHAR(255) NOT NULL,
//     -- サークル名
//     capacity INT NOT NULL,
//     -- 定員
//     version INT NOT NULL,
//     -- 最新バージョン
// );

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub(crate) struct CircleProtectionData {
    pub circle_id: String,
    pub name: String,
    pub capacity: i16,
    pub version: i32,
}

impl std::convert::TryFrom<CircleProtectionData> for Circle {
    type Error = anyhow::Error;

    fn try_from(data: CircleProtectionData) -> Result<Self, Self::Error> {
        let circle_id = CircleId::from_str(data.circle_id.as_str())?;

        let version = Version::try_from(data.version)
            .map_err(|_| anyhow::anyhow!("Failed to convert version from i32 to Version"))?;

        Ok(Circle {
            id: circle_id,
            name: data.name,
            capacity: data.capacity,
            version,
        })
    }
}

// try_from
impl std::convert::TryFrom<Circle> for CircleProtectionData {
    type Error = anyhow::Error;
    fn try_from(circle: Circle) -> Result<Self, Self::Error> {
        let circle_id = circle.id.to_string();
        let name = circle.name;
        let capacity = circle.capacity;
        let version = i32::try_from(circle.version)
            .map_err(|_| anyhow::anyhow!("Failed to convert version from Version to i32"))?;

        Ok(Self {
            circle_id,
            name,
            capacity,
            version,
        })
    }
}

impl CircleProtectionData {
    pub fn from_row(row: &sqlx::mysql::MySqlRow) -> Self {
        Self {
            circle_id: row.get("circle_id"),
            name: row.get("name"),
            capacity: row.get("capacity"),
            version: row.get("version"),
        }
    }
}
