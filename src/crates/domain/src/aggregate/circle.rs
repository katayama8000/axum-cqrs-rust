use super::value_object::{circle_id::CircleId, version::Version};
use anyhow::{Error, Result};
use event::CircleEvent;
pub mod event;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Circle {
    pub id: CircleId,
    pub name: String,
    pub capacity: i16,
    pub version: Version,
}

impl Circle {
    pub fn replay(events: Vec<CircleEvent>) -> Self {
        let mut state = match events.first() {
            Some(first_event) => Self::from_created_event(first_event.clone()),
            None => unreachable!("No events to initialize Circle"),
        };
        for event in events.iter().skip(1) {
            state.apply_event(event);
        }
        state
    }

    pub fn create(name: String, capacity: i16) -> Result<(Self, CircleEvent)> {
        Self::validate_capacity(capacity)?;
        let event = CircleEvent::build(CircleId::gen(), Version::new())
            .circle_created(name.clone(), capacity);
        let state = Self::from_created_event(event.clone());
        Ok((state, event))
    }

    pub fn update(
        self,
        name: Option<String>,
        capacity: Option<i16>,
    ) -> Result<(Self, CircleEvent)> {
        if let Some(new_capacity) = capacity {
            Self::validate_capacity(new_capacity)?;
        }
        let event = CircleEvent::build(self.id.clone(), self.version.clone())
            .circle_updated(name, capacity);
        let mut state = self.clone();
        state.apply_event(&event);
        Ok((state, event))
    }

    // Private helper methods for event sourcing

    fn from_created_event(event: CircleEvent) -> Self {
        match event.data {
            event::EventData::CircleCreated(event::CircleCreated { name, capacity }) => Self {
                id: event.circle_id,
                name,
                capacity,
                version: event.version,
            },
            _ => panic!("Invalid event for creation"),
        }
    }

    pub fn apply_event(&mut self, event: &CircleEvent) {
        match &event.data {
            event::EventData::CircleCreated(event::CircleCreated { name, capacity }) => {
                self.name = name.clone();
                self.capacity = *capacity;
                self.version = event.version.clone();
            }
            event::EventData::CircleUpdated(event::CircleUpdated { name, capacity }) => {
                if let Some(new_name) = name {
                    self.name = new_name.clone();
                }
                if let Some(new_capacity) = capacity {
                    self.capacity = *new_capacity;
                }
                self.version = event.version.clone();
            }
        }
    }

    // Getters

    pub fn name(&self) -> &str {
        &self.name
    }

    // utility methods

    fn validate_capacity(capacity: i16) -> Result<()> {
        if capacity < 3 {
            Err(Error::msg("Circle capacity must be 3 or more"))
        } else {
            Ok(())
        }
    }
}
