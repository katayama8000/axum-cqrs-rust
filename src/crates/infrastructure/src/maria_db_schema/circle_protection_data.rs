use std::str::FromStr;

use domain::aggregate::{
    circle::Circle,
    value_object::{circle_id::CircleId, version::Version},
};

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
    pub id: String,
    pub name: String,
    pub capacity: i16,
    pub version: u32,
}

impl std::convert::TryFrom<CircleProtectionData> for Circle {
    type Error = anyhow::Error;

    fn try_from(data: CircleProtectionData) -> Result<Self, Self::Error> {
        let circle_id = CircleId::from_str(data.id.as_str())?;

        let version = Version::from(data.version);

        Ok(Circle {
            id: circle_id,
            name: data.name,
            capacity: data.capacity,
            version,
        })
    }
}

impl std::convert::From<Circle> for CircleProtectionData {
    fn from(circle: Circle) -> Self {
        Self {
            id: circle.id.into(),
            name: circle.name,
            capacity: circle.capacity,
            version: circle.version.into(),
        }
    }
}
