use std::sync::Arc;

use crate::aggregate::circle::Circle;
use anyhow::Error;

#[mockall::automock]
#[async_trait::async_trait]
pub trait CircleDuplicateCheckerInterface: Send + Sync {
    async fn check_circle_duplicate(&self, circle: &Circle) -> Result<(), Error>;
}

pub trait HasCircleDuplicateCheckerInterface {
    fn circle_duplicate_checker(&self) -> Arc<dyn CircleDuplicateCheckerInterface + Send + Sync>;
}
