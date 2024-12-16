use std::sync::Arc;

use domain::interface::circle_duplicate_checker_interface::CircleDuplicateCheckerInterface;

pub(crate) struct CommandHandlerImpl {
    pub(crate) create_circle_repository: Arc<dyn CircleDuplicateCheckerInterface + Send + Sync>,
    pub(crate) create_circle_duplicate_checker:
        Arc<dyn CircleDuplicateCheckerInterface + Send + Sync>,
}
