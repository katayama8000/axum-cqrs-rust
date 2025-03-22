use std::sync::Arc;

use command::command_handler::{CommandHandler, HasCommandHandler};
use query::query_handler::{HasQueryHandler, QueryHandler};

#[derive(Clone)]
pub struct AppState {
    pub command_handler: Arc<dyn CommandHandler + Send + Sync>,
    pub query_handler: Arc<dyn QueryHandler + Send + Sync>,
}

impl AppState {
    pub fn new(
        command_handler: Arc<dyn CommandHandler + Send + Sync>,
        query_handler: Arc<dyn QueryHandler + Send + Sync>,
    ) -> Self {
        Self {
            command_handler,
            query_handler,
        }
    }
}

impl HasCommandHandler for AppState {
    fn command_handler(&self) -> Arc<dyn CommandHandler + Send + Sync> {
        self.command_handler.clone()
    }
}

impl HasQueryHandler for AppState {
    fn query_handler(&self) -> Arc<dyn QueryHandler + Send + Sync> {
        self.query_handler.clone()
    }
}
