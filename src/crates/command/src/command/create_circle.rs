use std::sync::Arc;

use serde::Deserialize;

use domain::{
    aggregate::circle::Circle,
    interface::command::circle_repository_interface::CircleRepositoryInterface,
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
    Input {
        circle_name,
        capacity,
    }: Input,
) -> Result<Output, Error> {
    let (circle, event) = Circle::create(circle_name, capacity).map_err(|_| Error::InvalidInput)?;

    circle_repository
        .store(None, vec![event])
        .await
        .map_err(|_| Error::Circle)?;

    Ok(Output {
        circle_id: circle.id.to_string(),
    })
}
