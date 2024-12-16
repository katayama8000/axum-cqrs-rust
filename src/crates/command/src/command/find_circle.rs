use anyhow::Result;
use domain::{
    aggregate::{
        circle::Circle,
        member::Member,
        value_object::circle_id::{self, CircleId},
    },
    interface::circle_repository_interface::CircleRepositoryInterface,
};
use serde::Deserialize;
use std::{str::FromStr, sync::Arc};

#[derive(Debug)]
pub enum Error {
    CircleNotFound,
}

impl From<Error> for String {
    fn from(e: Error) -> String {
        match e {
            Error::CircleNotFound => "circle not found".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Input {
    pub circle_id: String,
}

#[derive(Debug)]
pub struct Output {
    pub circle_name: String,
    pub owner_name: String,
    pub owner_age: i16,
    pub owner_grade: i16,
    pub owner_major: String,
}

pub async fn find_circle(
    circle_repository: Arc<dyn CircleRepositoryInterface + Send + Sync>,
    input: Input,
) -> Result<Output, Error> {
    let circle_id = CircleId::from_str(input.circle_id.as_str()).unwrap();
    // CircleRepositoryからcircle_idでCircleを探す
    match circle_repository.find_by_id(&circle_id).await {
        Ok(circle) => {
            let owner = circle.owner();
            Ok(Output {
                circle_name: circle.name().to_string(),
                owner_name: owner.name().to_string(),
                owner_age: owner.age().get(),
                owner_grade: owner.grade().get(),
                owner_major: owner.major().to_string(),
            })
        }
        Err(_) => Err(Error::CircleNotFound),
    }
}
