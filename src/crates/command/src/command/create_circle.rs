use std::{fmt::Display, sync::Arc};

use serde::Deserialize;

use domain::{
    aggregate::{
        circle::Circle,
        member::Member,
        value_object::{grade::Grade, major::Major},
    },
    interface::{
        circle_duplicate_checker_interface::CircleDuplicateCheckerInterface,
        circle_repository_interface::CircleRepositoryInterface,
    },
};

#[derive(Debug)]
pub enum Error {
    Circle,
    Duplicate,
    InvalidInput,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Circle => write!(f, "circle error"),
            Error::Duplicate => write!(f, "duplicate error"),
            Error::InvalidInput => write!(f, "invalid input error"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Input {
    pub circle_name: String,
    pub capacity: i16,
    pub owner_name: String,
    pub owner_age: i16,
    pub owner_grade: i16,
    pub owner_major: String,
}

#[derive(Debug)]
pub struct Output {
    pub circle_id: String,
    pub owner_id: String,
}

pub async fn handle(
    circle_repository: Arc<dyn CircleRepositoryInterface + Send + Sync>,
    circle_duplicate_checker: Arc<dyn CircleDuplicateCheckerInterface + Send + Sync>,
    Input {
        circle_name,
        capacity,
        owner_name,
        owner_age,
        owner_grade,
        owner_major,
    }: Input,
) -> Result<Output, Error> {
    // check input
    let grade = Grade::try_from(owner_grade).map_err(|_| Error::InvalidInput)?;
    let major = Major::from(owner_major.as_str());
    let owner = Member::create(owner_name, owner_age, grade, major);
    let circle = Circle::create(circle_name, owner, capacity).map_err(|_| Error::InvalidInput)?;

    // check duplicate
    circle_duplicate_checker
        .check_circle_duplicate(&circle)
        .await
        .map_err(|_| Error::Duplicate)?;

    // store
    circle_repository
        .store(None, &circle)
        .await
        .map_err(|_| Error::Circle)?;

    Ok(Output {
        circle_id: circle.id.to_string(),
        owner_id: circle.owner.id.to_string(),
    })
}
