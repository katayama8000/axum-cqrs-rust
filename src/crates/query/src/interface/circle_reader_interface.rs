use std::sync::Arc;

use domain::aggregate::{circle::Circle, value_object::circle_id::CircleId};

#[async_trait::async_trait]
pub trait CircleReaderInterface: Send + Sync {
    async fn get_circle(&self, id: CircleId) -> Result<Option<Circle>, anyhow::Error>;
    async fn list_circles(&self) -> Result<Vec<Circle>, anyhow::Error>;
}

pub trait HasCircleReader {
    fn circle_reader(&self) -> Arc<dyn CircleReaderInterface + Send + Sync>;
}
