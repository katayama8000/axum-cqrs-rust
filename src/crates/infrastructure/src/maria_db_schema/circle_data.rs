use std::str::FromStr;

use domain::aggregate::{
    circle::Circle,
    member::Member,
    value_object::{circle_id::CircleId, member_id::MemberId, version::Version},
};

use super::member_data::MemberData;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub(crate) struct CircleData {
    pub id: String,
    pub name: String,
    pub owner_id: String,
    pub owner: MemberData,
    pub capacity: i16,
    pub members: Vec<MemberData>,
    pub version: u32,
}

impl std::convert::TryFrom<CircleData> for Circle {
    type Error = anyhow::Error;

    fn try_from(data: CircleData) -> Result<Self, Self::Error> {
        let circle_id = CircleId::from_str(data.id.as_str())?;
        let owner_id = MemberId::from_str(data.owner_id.as_str())?;
        let members = data
            .members
            .into_iter()
            .map(|member_data| member_data.try_into())
            .collect::<Result<Vec<Member>, _>>()?;

        let owner = members
            .iter()
            .find(|member| member.id == owner_id)
            .ok_or_else(|| anyhow::Error::msg("Owner not found"))?
            .clone();

        let version = Version::from(data.version);

        Ok(Circle {
            id: circle_id,
            name: data.name,
            capacity: data.capacity,
            owner,
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
            owner_id: circle.owner.clone().id.into(),
            owner: MemberData::from(circle.owner),
            capacity: circle.capacity,
            members: circle.members.into_iter().map(MemberData::from).collect(),
            version: circle.version.into(),
        }
    }
}
