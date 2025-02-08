use super::{
    member::Member,
    value_object::{circle_id::CircleId, event_id, grade::Grade, major::Major, version::Version},
};
use anyhow::{Error, Result};
use event::Event;
pub mod event;

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

    pub fn create(name: String, owner: Member, capacity: i16) -> Result<(Self, Event)> {
        Self::validate_owner(&owner)?;
        Self::validate_capacity(capacity)?;
        let circle_id = CircleId::gen();
        let event_id = event_id::EventId::gen();

        let event = Event::new(
            // Add owner to circleCreated event
            event::EventData::CircleCreated(event::CircleCreated {
                name: name.clone(),
                capacity,
            }),
            circle_id.clone(),
            event_id,
            Version::new(),
        );
        let state = Self::create_from_created_event(event.clone());
        Ok((state, event))
    }

    pub fn update(self, name: Option<String>, capacity: Option<i16>) -> Result<(Self, Event)> {
        if let Some(new_capacity) = capacity {
            Self::validate_capacity(new_capacity)?;
        }

        let event_id = event_id::EventId::gen();
        let event = Event::new(
            event::EventData::CircleUpdated(event::CircleUpdated {
                name: name.clone(),
                capacity: capacity.clone(),
            }),
            self.id.clone(),
            event_id,
            self.version.next(),
        );
        let mut state = self.clone();
        state.apply_event(&event);
        Ok((state, event))
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

    // Private helper methods for event sourcing

    fn create_from_created_event(event: Event) -> Self {
        let dummy_member = Member::create(
            "dummy".to_string(),
            20,
            Grade::Third,
            Major::ComputerScience,
        );
        match event.data {
            event::EventData::CircleCreated(data) => Self {
                id: event.circle_id,
                name: data.name,
                capacity: data.capacity,
                owner: dummy_member,
                members: vec![],
                version: Version::new(),
            },
            _ => panic!("Invalid event data"),
        }
    }

    fn apply_event(&mut self, event: &Event) {
        match &event.data {
            event::EventData::CircleCreated(data) => {
                self.name = data.name.clone();
                self.capacity = data.capacity;
                self.version = event.version.clone();
            }
            event::EventData::CircleUpdated(data) => {
                self.name = data.name.clone().unwrap_or(self.name.clone());
                self.capacity = data.capacity.unwrap_or(self.capacity);
                self.version = event.version.clone();
            }
        }
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
