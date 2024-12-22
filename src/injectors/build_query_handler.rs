use std::sync::Arc;

use infrastructure::circle_reader::CircleReader;

use super::query_handler_impl::QueryHandlerImpl;

pub fn build_query_handler(db: sqlx::MySqlPool) -> QueryHandlerImpl {
    let circle_reader = Arc::new(CircleReader::new(db.clone()));

    QueryHandlerImpl { circle_reader }
}
