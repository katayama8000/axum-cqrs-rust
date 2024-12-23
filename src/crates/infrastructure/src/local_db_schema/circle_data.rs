use std::str::FromStr;

use domain::aggregate::{
    circle::Circle,
    member::Member,
    value_object::{
        circle_id::CircleId, grade::Grade, major::Major, member_id::MemberId, version::Version,
    },
};

use super::member_data::MemberData;

#[derive(serde::Deserialize, serde::Serialize)]
pub(crate) struct CircleData {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) owner: MemberData,
    pub(crate) capacity: i16,
    pub(crate) members: Vec<MemberData>,
    pub(crate) version: u32,
}

impl std::convert::From<Circle> for CircleData {
    fn from(circle: Circle) -> Self {
        CircleData {
            id: circle.id.into(),
            name: circle.name,
            owner: MemberData::from(circle.owner),
            capacity: circle.capacity,
            members: circle.members.into_iter().map(MemberData::from).collect(),
            version: circle.version.into(),
        }
    }
}

impl std::convert::TryFrom<CircleData> for Circle {
    type Error = anyhow::Error;

    fn try_from(data: CircleData) -> Result<Self, Self::Error> {
        Ok(Circle::reconstruct(
            CircleId::from_str(&data.id)?,
            data.name,
            Member::reconstruct(
                MemberId::from_str(&data.owner.id)?,
                data.owner.name,
                data.owner.age,
                Grade::try_from(data.owner.grade)?,
                Major::from(data.owner.major.as_str()),
                Version::from(data.version),
            ),
            data.capacity,
            data.members
                .into_iter()
                .map(Member::try_from)
                .collect::<Result<Vec<Member>, anyhow::Error>>()?,
            Version::from(data.version),
        ))
    }
}
