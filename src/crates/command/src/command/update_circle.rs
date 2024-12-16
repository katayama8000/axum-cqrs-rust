use std::{fmt::Display, str::FromStr, sync::Arc};

use serde::Deserialize;

use domain::{
    aggregate::value_object::circle_id::CircleId,
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
    pub circle_id: String,
    pub circle_name: Option<String>,
    pub capacity: Option<i16>,
}

#[derive(Debug)]
pub struct Output {
    pub circle_id: String,
}

pub async fn handle(
    circle_repository: Arc<dyn CircleRepositoryInterface + Send + Sync>,
    circle_duplicate_checker: Arc<dyn CircleDuplicateCheckerInterface + Send + Sync>,
    Input {
        circle_id,
        circle_name,
        capacity,
    }: Input,
) -> Result<Output, Error> {
    // check input
    let circle_id = CircleId::from_str(circle_id.as_str()).map_err(|_| Error::InvalidInput)?;

    // find the circle
    let circle = circle_repository
        .find_by_id(&circle_id)
        .await
        .map_err(|_| Error::Circle)?;

    // update
    let circle = circle.update(circle_name, capacity);

    // check duplicate
    circle_duplicate_checker
        .check_circle_duplicate(&circle)
        .await
        .map_err(|_| Error::Duplicate)?;

    // store
    // TODO: versioning
    circle_repository
        .store(None, &circle)
        .await
        .map_err(|_| Error::Circle)?;

    Ok(Output {
        circle_id: circle.id.to_string(),
    })
}
