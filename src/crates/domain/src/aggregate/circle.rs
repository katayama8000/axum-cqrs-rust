use super::{
    member::Member,
    value_object::{circle_id::CircleId, grade::Grade},
};
use anyhow::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Circle {
    pub id: CircleId,
    pub name: String,
    pub capacity: i16,
    pub owner: Member,
    pub members: Vec<Member>,
}

impl Circle {
    pub fn create(name: String, owner: Member, capacity: i16) -> Result<Self, Error> {
        if owner.grade != Grade::Third {
            return Err(Error::msg("Owner must be 3rd grade"));
        }

        if capacity < 3 {
            return Err(Error::msg("Circle capacity must be 3 or more"));
        }

        Ok(Circle {
            id: CircleId::gen(),
            name,
            owner,
            capacity,
            members: vec![],
        })
    }

    pub fn reconstruct(
        id: CircleId,
        name: String,
        owner: Member,
        capacity: i16,
        members: Vec<Member>,
    ) -> Self {
        Circle {
            id,
            name,
            owner,
            capacity,
            members,
        }
    }

    pub fn update(self, name: Option<String>, capacity: Option<i16>) -> Self {
        Circle {
            name: name.unwrap_or(self.name),
            capacity: capacity.unwrap_or(self.capacity),
            ..self
        }
    }

    fn is_full(&self) -> bool {
        self.members.len() + 1 >= self.capacity as usize
    }

    pub fn add_member(&mut self, member: Member) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::msg("Circle member is full"));
        }

        if member.grade == Grade::Fourth {
            return Err(Error::msg("4th grade can't join circle"));
        }

        self.members.push(member);
        Ok(())
    }

    pub fn remove_member(&mut self, member: &Member) -> Result<(), Error> {
        if self.owner.id == member.id {
            return Err(Error::msg("Owner can't be removed"));
        }
        self.members.retain(|m| m.id != member.id);
        Ok(())
    }

    pub fn graduate(&mut self) {
        self.members.retain(|m| m.grade != Grade::Fourth);
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
