use anyhow::Error;
use domain::{
    aggregate::{
        circle::Circle,
        value_object::{circle_id::CircleId, version},
    },
    interface::command::circle_repository_interface::CircleRepositoryInterface,
};

use super::db::Db;
use crate::local_db_schema::circle_data::CircleData;

#[derive(Clone, Debug)]
pub struct CircleRepository {
    db: Db,
}

impl CircleRepository {
    pub fn new() -> Self {
        Self { db: Db::new() }
    }
}

#[async_trait::async_trait]
impl CircleRepositoryInterface for CircleRepository {
    async fn find_by_id(&self, _circle_id: &CircleId) -> Result<Circle, Error> {
        // match self.db.get::<CircleData, _>(&circle_id.to_string())? {
        //     Some(data) => Ok(Circle::try_from(data)?),
        //     None => Err(Error::msg("Circle not found")),
        // }

        unimplemented!()
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

    async fn delete(&self, circle: &Circle) -> Result<(), Error> {
        match self.db.get::<CircleData, _>(&circle.id.to_string())? {
            Some(_) => self.db.remove(circle.id.to_string()),
            None => Err(Error::msg("Circle not found")),
        }
    }
}

#[cfg(test)]
mod tests {
    use domain::{
        aggregate::{
            circle::{event::Event, member::Member, Circle},
            value_object::{grade::Grade, major::Major},
        },
        interface::command::circle_repository_interface::CircleRepositoryInterface,
    };

    use super::CircleRepository;

    // ignore this test
    #[ignore]
    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let (mut circle1, _event) = build_circle()?;
        let repository = CircleRepository::new();
        assert!(repository.find_by_id(&circle1.id).await.is_err());
        repository.store(None, &circle1).await?;
        assert_eq!(repository.find_by_id(&circle1.id).await?, circle1);
        circle1.name = "circle_name2".to_string();
        repository.store(None, &circle1).await?;
        assert_eq!(repository.find_by_id(&circle1.id).await?, circle1);
        repository.delete(&circle1).await?;
        assert!(repository.find_by_id(&circle1.id).await.is_err());
        Ok(())
    }

    fn build_circle() -> anyhow::Result<(Circle, Event)> {
        Circle::create(
            "Music club".to_string(),
            Member::create("member_name1".to_string(), 21, Grade::Third, Major::Art),
            3,
        )
        .map(|(circle, event)| (circle, event))
    }
}
