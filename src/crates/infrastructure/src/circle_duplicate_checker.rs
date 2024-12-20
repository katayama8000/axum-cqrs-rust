use async_trait::async_trait;
use domain::{
    aggregate::circle::Circle,
    interface::circle_duplicate_checker_interface::CircleDuplicateCheckerInterface,
};
use sqlx::MySqlPool;

#[derive(Clone, Debug)]
pub struct CircleDuplicateCheckerWithMySql {
    db: MySqlPool,
}

impl CircleDuplicateCheckerWithMySql {
    pub fn new(db: MySqlPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl CircleDuplicateCheckerInterface for CircleDuplicateCheckerWithMySql {
    async fn check_circle_duplicate(
        &self,
        circle: &Circle,
    ) -> Result<(), domain::interface::circle_duplicate_checker_interface::Error> {
        let query = "SELECT * FROM circles WHERE name = ?";
        let record = sqlx::query(query)
            .bind(circle.name())
            .fetch_optional(&self.db)
            .await
            .map_err(Into::into)
            .map_err(domain::interface::circle_duplicate_checker_interface::Error)?;

        if record.is_some() {
            return Err(
                domain::interface::circle_duplicate_checker_interface::Error(Box::new(
                    std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Circle already exists"),
                )),
            );
        }
        Ok(())
    }
}
