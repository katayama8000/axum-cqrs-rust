use super::{
    member::Member,
    value_object::{circle_id::CircleId, grade::Grade, version::Version},
};
use anyhow::{Error, Result};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Circle {
    pub id: CircleId,
    pub name: String,
    pub capacity: i16,
    pub owner: Member,
    pub members: Vec<Member>,
    pub version: Version,
}

impl Circle {
    pub fn create(name: String, owner: Member, capacity: i16) -> Result<Self> {
        Self::validate_owner(&owner)?;
        Self::validate_capacity(capacity)?;

        Ok(Self {
            id: CircleId::gen(),
            name,
            owner,
            capacity,
            members: vec![],
            version: Version::new(),
        })
    }

    pub fn reconstruct(
        id: CircleId,
        name: String,
        owner: Member,
        capacity: i16,
        members: Vec<Member>,
        version: Version,
    ) -> Self {
        Self {
            id,
            name,
            owner,
            capacity,
            members,
            version,
        }
    }

    pub fn update(self, name: Option<String>, capacity: Option<i16>) -> Result<Self> {
        if let Some(new_capacity) = capacity {
            Self::validate_capacity(new_capacity)?;
        }

        Ok(Self {
            name: name.unwrap_or(self.name),
            capacity: capacity.unwrap_or(self.capacity),
            version: self.version.next(),
            ..self
        })
    }

    pub fn add_member(&mut self, member: Member) -> Result<()> {
        self.validate_member(&member)?;

        if self.is_full() {
            return Err(Error::msg("Circle is at full capacity"));
        }

        self.members.push(member);
        self.version = self.version.next();
        Ok(())
    }

    pub fn remove_member(&mut self, member: &Member) -> Result<()> {
        if self.owner.id == member.id {
            return Err(Error::msg("Owner cannot be removed"));
        }

        self.members.retain(|m| m.id != member.id);
        self.version = self.version.next();
        Ok(())
    }

    pub fn graduate(&mut self) {
        self.members.retain(|m| m.grade != Grade::Fourth);
        self.version = self.version.next();
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    // Private helper methods
    fn is_full(&self) -> bool {
        self.members.len() + 1 >= self.capacity as usize
    }

    fn validate_owner(owner: &Member) -> Result<()> {
        if owner.grade != Grade::Third {
            Err(Error::msg("Owner must be in 3rd grade"))
        } else {
            Ok(())
        }
    }

    fn validate_capacity(capacity: i16) -> Result<()> {
        if capacity < 3 {
            Err(Error::msg("Circle capacity must be 3 or more"))
        } else {
            Ok(())
        }
    }

    fn validate_member(&self, member: &Member) -> Result<()> {
        if member.grade == Grade::Fourth {
            return Err(Error::msg("4th grade members cannot join the circle"));
        }
        Ok(())
    }
}
