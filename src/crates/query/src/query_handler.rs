use std::sync::Arc;

use crate::{
    interface::get_circle_reader_interface::HasGetCircleReader,
    query::get_circle::{self, handle},
};

#[async_trait::async_trait]
pub trait QueryHandler: HasGetCircleReader {
    async fn get_circle(
        &self,
        input: get_circle::Input,
    ) -> Result<get_circle::Output, anyhow::Error> {
        handle(self.get_circle_reader(), input).await
    }

    // async fn list_circles() -> Result<list_circles::Output, list_circles::Error>;
}

pub trait HasQueryHandler {
    fn query_handler(&self) -> Arc<dyn QueryHandler + Send + Sync>;
}
