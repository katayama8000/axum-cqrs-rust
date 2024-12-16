use std::sync::Arc;

use domain::interface::{
    circle_duplicate_checker_interface::HasCircleDuplicateCheckerInterface,
    circle_repository_interface::HasCircleRepositoryInterface,
};

use crate::command::create_circle;

#[async_trait::async_trait]
pub trait CommandHandler:
    HasCircleRepositoryInterface + HasCircleDuplicateCheckerInterface
{
    async fn create_circle(
        &self,
        input: create_circle::Input,
    ) -> Result<create_circle::Output, create_circle::Error> {
        create_circle::handle(
            self.circle_repository(),
            self.circle_duplicate_checker(),
            input,
        )
        .await
    }
}

pub trait HasCommandHandler {
    fn command_handler(&self) -> Arc<dyn CommandHandler + Send + Sync>;
}
