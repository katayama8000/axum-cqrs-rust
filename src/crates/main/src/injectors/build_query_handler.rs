use std::sync::Arc;

use infrastructure::circle_reader::CircleReader;

use super::query_handler_impl::QueryHandlerImpl;

pub fn build_query_handler(redis_client: redis::Client) -> QueryHandlerImpl {
    let circle_reader = Arc::new(CircleReader::new(redis_client));
    QueryHandlerImpl { circle_reader }
}
