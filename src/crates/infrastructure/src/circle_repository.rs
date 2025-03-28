use domain::{
    aggregate::{
        circle::Circle,
        value_object::{circle_id::CircleId, version},
    },
    interface::command::circle_repository_interface::CircleRepositoryInterface,
};
use sqlx::Row;

use super::maria_db_schema::circle_data::CircleData;

#[derive(Clone, Debug)]
pub struct CircleRepository {
    db: sqlx::MySqlPool,
}

impl CircleRepository {
    pub fn new(db: sqlx::MySqlPool) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl CircleRepositoryInterface for CircleRepository {
    async fn find_by_id(&self, circle_id: &CircleId) -> Result<Circle, anyhow::Error> {
        tracing::info!("find_circle_by_id : {:?}", circle_id);
        let circle_query =
            sqlx::query("SELECT * FROM circles WHERE id = ?").bind(circle_id.to_string());

        let circle_row = circle_query.fetch_one(&self.db).await.map_err(|e| {
            eprintln!("Failed to fetch circle by id: {:?}", e);
            anyhow::Error::msg("Failed to fetch circle by id")
        })?;

        let circle_data = CircleData {
            id: circle_row.get::<String, _>("id"),
            name: circle_row.get::<String, _>("name"),
            capacity: circle_row.get::<i16, _>("capacity"),
            version: circle_row.get::<u32, _>("version"),
        };

        Ok(Circle::try_from(circle_data)?)
    }

    async fn store(
        &self,
        version: Option<version::Version>,
        _circle: &Circle,
    ) -> Result<(), anyhow::Error> {
        match version {
            Some(_version) => {
                unimplemented!("update_circle")
            }
            None => {
                unimplemented!("create_circle")
            }
        }
    }

    async fn delete(&self, _circle: &Circle) -> Result<(), anyhow::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {}
