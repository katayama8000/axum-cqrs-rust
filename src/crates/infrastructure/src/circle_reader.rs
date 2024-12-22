use domain::aggregate::{circle::Circle, value_object::circle_id::CircleId};
use query::interface::circle_reader_interface::CircleReaderInterface;

use anyhow::Error;
use sqlx::MySqlPool;

#[derive(Clone, Debug)]
pub struct CircleReader {
    db: MySqlPool,
}

impl CircleReader {
    pub fn new(db: sqlx::MySqlPool) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl CircleReaderInterface for CircleReader {
    async fn get_circle(&self, _id: CircleId) -> Result<Option<Circle>, Error> {
        unimplemented!("get_circle")
    }

    async fn list_circles(&self) -> Result<Vec<Circle>, Error> {
        unimplemented!("list_circles")
    }
}
