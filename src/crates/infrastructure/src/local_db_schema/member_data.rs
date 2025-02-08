use std::str::FromStr;

use domain::aggregate::{
    circle::member::Member,
    value_object::member_id::MemberId,
    value_object::{grade::Grade, major::Major, version::Version},
};

#[derive(serde::Deserialize, serde::Serialize)]
pub(crate) struct MemberData {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) age: i16,
    pub(crate) grade: i16,
    pub(crate) major: String,
    pub(crate) version: u32,
}

impl std::convert::From<Member> for MemberData {
    fn from(value: Member) -> Self {
        Self {
            id: value.id.into(),
            name: value.name,
            age: value.age,
            grade: value.grade.into(),
            major: value.major.into(),
            version: value.version.into(),
        }
    }
}

impl std::convert::TryFrom<MemberData> for Member {
    type Error = anyhow::Error;

    fn try_from(value: MemberData) -> Result<Self, Self::Error> {
        Ok(Member::reconstruct(
            MemberId::from_str(value.id.as_str())?,
            value.name,
            value.age,
            Grade::try_from(value.grade)?,
            Major::from(value.major.as_str()),
            Version::from(value.version),
        ))
    }
}
