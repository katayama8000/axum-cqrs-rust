use domain::{
    aggregate::{
        circle::Circle,
        value_object::{circle_id::CircleId, version},
    },
    interface::circle_repository_interface::CircleRepositoryInterface,
};
use sqlx::Row;

use super::db_data::{circle_data::CircleData, member_data::MemberData};

#[derive(Clone, Debug)]
pub struct CircleRepositoryWithMySql {
    db: sqlx::MySqlPool,
}

impl CircleRepositoryWithMySql {
    pub fn new(db: sqlx::MySqlPool) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl CircleRepositoryInterface for CircleRepositoryWithMySql {
    async fn find_all(&self) -> Result<Vec<Circle>, anyhow::Error> {
        tracing::info!("find_all_circles");
        let circle_query = sqlx::query("SELECT * FROM circles");

        let circle_rows = circle_query.fetch_all(&self.db).await.map_err(|e| {
            eprintln!("Failed to fetch circles: {:?}", e);
            anyhow::Error::msg("Failed to fetch circles")
        })?;

        let mut circles = Vec::new();
        for circle_row in circle_rows {
            let member_query = sqlx::query("SELECT * FROM members WHERE circle_id = ?")
                .bind(circle_row.get::<i16, _>("id"));

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
            };

            circles.push(Circle::try_from(circle_data)?);
        }

        Ok(circles)
    }

    async fn find_by_id(&self, circle_id: &CircleId) -> Result<Circle, anyhow::Error> {
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
                unimplemented!("store_circle with version")
            }
            None => {
                unimplemented!("store_circle without version")
            }
        }
    }

    async fn delete(&self, _circle: &Circle) -> Result<(), anyhow::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {}
