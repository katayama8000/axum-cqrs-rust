use std::sync::Arc;

use infrastructure::{
    circle_duplicate_checker::CircleDuplicateChecker, 
    circle_repository::CircleRepository,
    event_publisher::EventPublisher,
};

use super::command_handler_impl::CommandHandlerImpl;

pub fn build_command_handler(
    db: sqlx::MySqlPool,
    event_publisher: Arc<dyn EventPublisher>,
) -> CommandHandlerImpl {
    let circle_repository = Arc::new(CircleRepository::new(
        db.clone(), 
        event_publisher
    ));
    let circle_duplicate_checker = Arc::new(CircleDuplicateChecker::new(db.clone()));

    CommandHandlerImpl {
        circle_repository,
        circle_duplicate_checker,
    }
}
