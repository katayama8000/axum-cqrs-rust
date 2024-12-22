use std::sync::Arc;

use crate::query::list_circles::Output;

#[async_trait::async_trait]
pub trait ListCirclesReaderInterface {
    async fn list_circles(&self) -> Result<Output, anyhow::Error>;
}

pub trait HasListCirclesReader {
    fn list_circles_reader(&self) -> Arc<dyn ListCirclesReaderInterface + Send + Sync>;
}
