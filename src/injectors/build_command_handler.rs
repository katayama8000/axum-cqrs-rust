use std::sync::Arc;

use infrastructure::{
    circle_duplicate_checker_with_my_sql::CircleDuplicateCheckerWithMySql,
    circle_repository_with_my_sql::CircleRepositoryWithMySql,
};

use super::command_handler_impl::CommandHandlerImpl;

pub fn build_command_handler(db: sqlx::MySqlPool) -> CommandHandlerImpl {
    let circle_repository = Arc::new(CircleRepositoryWithMySql::new(db.clone()));
    let circle_duplicate_checker = Arc::new(CircleDuplicateCheckerWithMySql::new(db.clone()));

    CommandHandlerImpl {
        circle_repository,
        circle_duplicate_checker,
    }
}
