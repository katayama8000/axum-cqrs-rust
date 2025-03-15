use std::str::FromStr;

use domain::aggregate::{
    circle::Circle,
    value_object::{circle_id::CircleId, version::Version},
};

use super::member_data::MemberData;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub(crate) struct CircleData {
    pub id: String,
    pub name: String,
    pub capacity: i16,
    pub members: Vec<MemberData>,
    pub version: u32,
}

impl std::convert::TryFrom<CircleData> for Circle {
    type Error = anyhow::Error;

    fn try_from(data: CircleData) -> Result<Self, Self::Error> {
        let circle_id = CircleId::from_str(data.id.as_str())?;
        let members = data
            .members
            .into_iter()
            .map(|member_data| member_data.try_into())
            .collect::<Result<Vec<Member>, _>>()?;

        let version = Version::from(data.version);

        Ok(Circle {
            id: circle_id,
            name: data.name,
            capacity: data.capacity,
            members,
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
            members: circle.members.into_iter().map(MemberData::from).collect(),
            version: circle.version.into(),
        }
    }
}
