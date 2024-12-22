use std::sync::Arc;

use crate::{
    interface::circle_reader_interface::HasCircleReader,
    query::{
        get_circle::{self},
        list_circles::{self},
    },
};

#[async_trait::async_trait]
pub trait QueryHandler: HasCircleReader {
    async fn get_circle(
        &self,
        input: get_circle::Input,
    ) -> Result<get_circle::Output, anyhow::Error> {
        get_circle::handle(self.circle_reader(), input).await
    }

    async fn list_circles(&self) -> Result<list_circles::Output, anyhow::Error> {
        list_circles::handle(self.circle_reader(), list_circles::Input {}).await
    }
}

pub trait HasQueryHandler {
    fn query_handler(&self) -> Arc<dyn QueryHandler + Send + Sync>;
}
