use domain::{
    aggregate::{circle::Circle, value_object::circle_id::CircleId},
    interface::query::circle_reader_interface::CircleReaderInterface,
};

use crate::maria_db_schema::circle_protection_data::CircleProtectionData;

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
        let circle_query = sqlx::query("SELECT * FROM circle_projections WHERE circle_id = ?")
            .bind(circle_id.to_string());

        let circle_row = circle_query
            .fetch_one(&self.db)
            .await
            .map_err(|_| anyhow::Error::msg("Failed to fetch circle_projections by id"))?;
        let v = CircleProtectionData::from_row(&circle_row);
        Ok(Some(Circle::try_from(v)?))
    }

    async fn list_circles(&self) -> Result<Vec<Circle>, Error> {
        tracing::info!("list_circles");
        let circle_query = sqlx::query("SELECT * FROM circle_projections");

        let circle_rows = circle_query
            .fetch_all(&self.db)
            .await
            .map_err(|_| anyhow::Error::msg("Failed to fetch circle_projections"))?;

        let circles = circle_rows
            .iter()
            .map(|row| {
                let circle_data = CircleProtectionData::from_row(row);
                Circle::try_from(circle_data)
            })
            .collect::<Result<Vec<Circle>, Error>>()?;

        Ok(circles)
    }
}
