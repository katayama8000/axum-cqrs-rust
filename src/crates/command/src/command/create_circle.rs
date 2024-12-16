use std::sync::Arc;

use anyhow::Result;
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
}

impl From<Error> for String {
    fn from(e: Error) -> String {
        match e {
            Error::Circle => "circle error".to_string(),
            Error::Duplicate => "duplicate error".to_string(),
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
    let grade = Grade::try_from(owner_grade).unwrap();
    let major = Major::from(owner_major.as_str());
    let owner = Member::new(owner_name, owner_age, grade, major);
    let circle = Circle::new(circle_name, owner, capacity).unwrap();

    match {
        circle_duplicate_checker
            .check_circle_duplicate(&circle)
            .await
    } {
        Ok(_) => {}
        Err(_) => {
            return Err(Error::Duplicate);
        }
    }

    circle_repository.store(None, &circle).await.unwrap();

    Ok(Output {
        circle_id: circle.id.to_string(),
        owner_id: circle.owner.id.to_string(),
    })
}
