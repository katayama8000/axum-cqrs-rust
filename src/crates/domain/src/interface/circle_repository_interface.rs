use std::sync::Arc;

use crate::aggregate::{
    circle::Circle,
    value_object::{circle_id::CircleId, version::Version},
};

pub struct Error(pub Box<dyn std::error::Error + Send + Sync>);

#[mockall::automock]
#[async_trait::async_trait]
pub trait CircleRepositoryInterface: Send + Sync {
    async fn find_by_id(&self, circle_id: &CircleId) -> Result<Circle, Error>;
    async fn store(&self, current: Option<Version>, circle: &Circle) -> Result<(), Error>;
    async fn delete(&self, circle: &Circle) -> Result<(), Error>;
}

pub trait HasCircleRepositoryInterface {
    fn circle_repository(&self) -> Arc<dyn CircleRepositoryInterface + Send + Sync>;
}
