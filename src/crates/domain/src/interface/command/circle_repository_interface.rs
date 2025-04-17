use std::sync::Arc;

use crate::aggregate::{
    circle::Circle,
    value_object::{circle_id::CircleId, version::Version},
};
use anyhow::Error;

#[mockall::automock]
#[async_trait::async_trait]
pub trait CircleRepositoryInterface: Send + Sync {
    async fn find_by_id(&self, circle_id: &CircleId) -> Result<Circle, Error>;
    async fn store(
        &self,
        current_version: Option<Version>,
        events: Vec<crate::aggregate::circle::event::CircleEvent>,
    ) -> Result<(), Error>;
}

pub trait HasCircleRepositoryInterface {
    fn circle_repository(&self) -> Arc<dyn CircleRepositoryInterface + Send + Sync>;
}
