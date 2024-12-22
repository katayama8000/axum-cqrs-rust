use query::{
    interface::{
        get_circle_reader_interface::{GetCircleReaderInterface, HasGetCircleReader},
        list_circles_reader_interface::{HasListCirclesReader, ListCirclesReaderInterface},
    },
    query_handler::QueryHandler,
};
use std::sync::Arc;

pub(crate) struct QueryHandlerImpl {
    pub(crate) get_circle_reader: Arc<dyn GetCircleReaderInterface + Send + Sync>,
    pub(crate) list_circles_reader: Arc<dyn ListCirclesReaderInterface + Send + Sync>,
}

impl HasGetCircleReader for QueryHandlerImpl {
    fn get_circle_reader(&self) -> Arc<dyn GetCircleReaderInterface + Send + Sync> {
        self.get_circle_reader.clone()
    }
}

impl HasListCirclesReader for QueryHandlerImpl {
    fn list_circles_reader(&self) -> Arc<dyn ListCirclesReaderInterface + Send + Sync> {
        self.list_circles_reader.clone()
    }
}

impl QueryHandler for QueryHandlerImpl {}
