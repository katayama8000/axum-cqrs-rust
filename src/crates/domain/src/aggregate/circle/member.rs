use crate::aggregate::value_object::{
    grade::Grade, major::Major, member_id::MemberId, version::Version,
};

mod event;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Member {
    pub id: MemberId,
    pub name: String,
    pub age: i16,
    pub grade: Grade,
    pub major: Major,
    pub version: Version,
}

impl Member {
    pub fn create(name: String, age: i16, grade: Grade, major: Major) -> Self {
        Self {
            id: MemberId::gen(),
            name,
            age,
            grade,
            major,
            version: Version::new(),
        }
    }

    pub fn reconstruct(
        id: MemberId,
        name: String,
        age: i16,
        grade: Grade,
        major: Major,
        version: Version,
    ) -> Self {
        Self {
            id,
            name,
            age,
            grade,
            major,
            version,
        }
    }

    pub fn is_adult(&self) -> bool {
        self.age >= 20
    }
}
