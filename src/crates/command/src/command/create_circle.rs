use std::sync::Arc;

use serde::Deserialize;

use domain::{
    aggregate::{
        circle::Circle,
        value_object::{grade::Grade, major::Major},
    },
    interface::command::{
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

#[derive(Debug, Deserialize)]
pub struct Input {
    pub circle_name: String,
    pub capacity: i16,
}

#[derive(Debug)]
pub struct Output {
    pub circle_id: String,
}

pub async fn handle(
    circle_repository: Arc<dyn CircleRepositoryInterface + Send + Sync>,
    circle_duplicate_checker: Arc<dyn CircleDuplicateCheckerInterface + Send + Sync>,
    Input {
        circle_name,
        capacity,
    }: Input,
) -> Result<Output, Error> {
    // check input
    let grade = Grade::try_from(owner_grade).map_err(|_| Error::InvalidInput)?;
    let major = Major::from(owner_major.as_str());

    // create
    let (circle, _event) =
        Circle::create(circle_name, capacity).map_err(|_| Error::InvalidInput)?;

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
    })
}
