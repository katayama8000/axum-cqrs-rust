use domain::{
    aggregate::{circle::Circle, value_object::circle_id::CircleId},
    interface::query::circle_reader_interface::CircleReaderInterface,
};
use sqlx::Row;

use crate::maria_db_schema::circle_data::CircleData;

use anyhow::Error;

#[derive(Clone, Debug)]
pub struct CircleReader {
    db: sqlx::MySqlPool,
}

impl CircleReader {
    pub fn new(db: sqlx::MySqlPool) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl CircleReaderInterface for CircleReader {
    async fn get_circle(&self, circle_id: CircleId) -> Result<Option<Circle>, Error> {
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

        Ok(Some(Circle::try_from(circle_data)?))
    }

    async fn list_circles(&self) -> Result<Vec<Circle>, Error> {
        let circle_query = sqlx::query("SELECT * FROM circles");

        let circle_rows = circle_query.fetch_all(&self.db).await.map_err(|e| {
            eprintln!("Failed to fetch circles: {:?}", e);
            anyhow::Error::msg("Failed to fetch circles")
        })?;

        let circles: Vec<CircleData> = circle_rows
            .into_iter()
            .map(|circle| {
                Ok(CircleData {
                    id: circle.get::<String, _>("id"),
                    name: circle.get::<String, _>("name"),
                    capacity: circle.get::<i16, _>("capacity"),
                    version: circle.get::<u32, _>("version"),
                })
            })
            .collect::<Result<Vec<CircleData>, Error>>()?;

        Ok(circles
            .into_iter()
            .map(|circle_data| Circle::try_from(circle_data))
            .collect::<Result<Vec<Circle>, Error>>()?)
    }
}
