use std::sync::Arc;

use domain::aggregate::value_object::circle_id::CircleId;

use crate::query::get_circle::Output;

#[async_trait::async_trait]
pub trait GetCircleReaderInterface {
    async fn get_circle(&self, id: CircleId) -> Result<Output, anyhow::Error>;
}

pub trait HasGetCircleReader {
    fn get_circle_reader(&self) -> Arc<dyn GetCircleReaderInterface + Send + Sync>;
}
