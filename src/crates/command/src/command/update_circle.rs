use std::{str::FromStr, sync::Arc};

use serde::Deserialize;

use domain::{
    aggregate::value_object::{circle_id::CircleId, version::Version},
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
    VersionMismatch,
}

#[derive(Debug, Deserialize)]
pub struct Input {
    pub circle_id: String,
    pub circle_name: Option<String>,
    pub capacity: Option<i16>,
    pub version: u32,
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
        version,
    }: Input,
) -> Result<Output, Error> {
    // check input
    let circle_id = CircleId::from_str(circle_id.as_str()).map_err(|_| Error::InvalidInput)?;
    let version = Version::from(version);

    // find the circle
    let circle = circle_repository
        .find_by_id(&circle_id)
        .await
        .map_err(|_| Error::Circle)?;

    // update
    let circle = circle
        .update(circle_name, capacity)
        .map_err(|_| Error::InvalidInput)?;

    // check duplicate
    circle_duplicate_checker
        .check_circle_duplicate(&circle)
        .await
        .map_err(|_| Error::Duplicate)?;

    // check version
    if circle.version != version {
        return Err(Error::VersionMismatch);
    }

    // store
    circle_repository
        .store(Some(version), &circle)
        .await
        .map_err(|_| Error::Circle)?;

    Ok(Output {
        circle_id: circle.id.to_string(),
    })
}
