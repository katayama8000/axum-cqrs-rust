use std::sync::Arc;

use infrastructure::{
    circle_duplicate_checker::CircleDuplicateChecker, circle_repository::CircleRepository,
};

use super::command_handler_impl::CommandHandlerImpl;

pub fn build_command_handler(db: sqlx::MySqlPool) -> CommandHandlerImpl {
    let circle_repository = Arc::new(CircleRepository::new(db.clone()));
    let circle_duplicate_checker = Arc::new(CircleDuplicateChecker::new(db.clone()));

    CommandHandlerImpl {
        circle_repository,
        circle_duplicate_checker,
    }
}
