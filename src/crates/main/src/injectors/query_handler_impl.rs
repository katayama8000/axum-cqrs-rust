use domain::interface::query::circle_reader_interface::{CircleReaderInterface, HasCircleReader};
use query::query_handler::QueryHandler;
use std::sync::Arc;

pub(crate) struct QueryHandlerImpl {
    pub(crate) circle_reader: Arc<dyn CircleReaderInterface + Send + Sync>,
}

impl HasCircleReader for QueryHandlerImpl {
    fn circle_reader(&self) -> Arc<dyn CircleReaderInterface + Send + Sync> {
        self.circle_reader.clone()
    }
}

impl QueryHandler for QueryHandlerImpl {}
