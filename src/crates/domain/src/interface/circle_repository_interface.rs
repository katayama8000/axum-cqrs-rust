use std::sync::Arc;

use crate::aggregate::{
    circle::{event::Event, Circle},
    value_object::{circle_id::CircleId, version::Version},
};
use anyhow::Error;

#[mockall::automock]
#[async_trait::async_trait]
pub trait CircleRepositoryInterface: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Circle>, Error>;
    async fn find_by_id(&self, circle_id: &CircleId) -> Result<Circle, Error>;
    async fn store(&self, current: Option<Version>, events: Vec<Event>) -> Result<(), Error>;
    async fn delete(&self, circle: &Circle) -> Result<(), Error>;
}

pub trait HasCircleRepositoryInterface {
    fn circle_repository(&self) -> Arc<dyn CircleRepositoryInterface + Send + Sync>;
}
