use std::str::FromStr;

use domain::aggregate::{
    circle::Circle,
    value_object::{circle_id::CircleId, version::Version},
};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub(crate) struct CircleData {
    pub id: String,
    pub name: String,
    pub capacity: i16,
    pub version: u32,
}

impl std::convert::TryFrom<CircleData> for Circle {
    type Error = anyhow::Error;

    fn try_from(data: CircleData) -> Result<Self, Self::Error> {
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

impl std::convert::From<Circle> for CircleData {
    fn from(circle: Circle) -> Self {
        Self {
            id: circle.id.into(),
            name: circle.name,
            capacity: circle.capacity,
            version: circle.version.into(),
        }
    }
}
