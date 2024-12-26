use domain::{
    aggregate::{circle::Circle, value_object::circle_id::CircleId},
    interface::query::circle_reader_interface::CircleReaderInterface,
};
use sqlx::Row;

use crate::maria_db_schema::{circle_data::CircleData, member_data::MemberData};

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

        let member_query =
            sqlx::query("SELECT * FROM members WHERE circle_id = ?").bind(circle_id.to_string());

        let members_row = member_query.fetch_all(&self.db).await.map_err(|e| {
            eprintln!("Failed to fetch members by circle id: {:?}", e);
            anyhow::Error::msg("Failed to fetch members by circle id")
        })?;

        let members: Vec<MemberData> = members_row
            .into_iter()
            .map(|member| MemberData {
                id: member.get::<String, _>("id"),
                name: member.get::<String, _>("name"),
                age: member.get::<i16, _>("age"),
                grade: member.get::<i16, _>("grade"),
                major: member.get::<String, _>("major"),
                version: member.get::<u32, _>("version"),
            })
            .collect();

        let owner: MemberData = members
            .iter()
            .find(|member| member.id == circle_row.get::<String, _>("owner_id"))
            .ok_or_else(|| anyhow::Error::msg("Owner not found"))?
            .clone();

        let circle_data = CircleData {
            id: circle_row.get::<String, _>("id"),
            name: circle_row.get::<String, _>("name"),
            owner_id: circle_row.get::<String, _>("owner_id"),
            owner,
            capacity: circle_row.get::<i16, _>("capacity"),
            members,
            version: circle_row.get::<u32, _>("version"),
        };

        Ok(Some(Circle::try_from(circle_data)?))
    }

    async fn list_circles(&self) -> Result<Vec<Circle>, Error> {
        unimplemented!("list_circles")
    }
}
