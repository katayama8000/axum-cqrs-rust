use std::sync::Arc;

use command::command_handler::CommandHandler;
use domain::interface::{
    command::circle_duplicate_checker_interface::{
        CircleDuplicateCheckerInterface, HasCircleDuplicateCheckerInterface,
    },
    command::circle_repository_interface::{
        CircleRepositoryInterface, HasCircleRepositoryInterface,
    },
};

pub(crate) struct CommandHandlerImpl {
    pub(crate) circle_repository: Arc<dyn CircleRepositoryInterface + Send + Sync>,
    pub(crate) circle_duplicate_checker: Arc<dyn CircleDuplicateCheckerInterface + Send + Sync>,
}

impl HasCircleDuplicateCheckerInterface for CommandHandlerImpl {
    fn circle_duplicate_checker(&self) -> Arc<dyn CircleDuplicateCheckerInterface + Send + Sync> {
        self.circle_duplicate_checker.clone()
    }
}

impl HasCircleRepositoryInterface for CommandHandlerImpl {
    fn circle_repository(&self) -> Arc<dyn CircleRepositoryInterface + Send + Sync> {
        self.circle_repository.clone()
    }
}

impl CommandHandler for CommandHandlerImpl {}
