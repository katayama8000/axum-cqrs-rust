use domain::{
    aggregate::{circle::Circle, value_object::circle_id::CircleId},
    interface::query::circle_reader_interface::CircleReaderInterface,
};

use crate::local_db_schema::circle_data::CircleData;

use super::db::Db;
use anyhow::Error;

#[derive(Clone, Debug)]
pub struct CircleReader {
    db: Db,
}

impl CircleReader {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl CircleReaderInterface for CircleReader {
    async fn get_circle(&self, id: CircleId) -> Result<Option<Circle>, Error> {
        match self.db.get::<CircleData, _>(&id.to_string())? {
            Some(data) => Ok(Some(Circle::try_from(data)?)),
            None => Err(Error::msg("Circle not found")),
        }
    }

    async fn list_circles(&self) -> Result<Vec<Circle>, Error> {
        unimplemented!("list_circles")
    }
}
