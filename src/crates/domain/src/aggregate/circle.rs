use super::value_object::{circle_id::CircleId, event_id, version::Version};
use anyhow::{Error, Result};
use event::Event;
pub mod event;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Circle {
    pub id: CircleId,
    pub name: String,
    pub capacity: i16,
    pub version: Version,
}

impl Circle {
    pub fn reconstruct(events: Vec<Event>) -> Self {
        let mut state = match events.first() {
            Some(first_event) => Self::create_from_created_event(first_event.clone()),
            None => unreachable!("No events to reconstruct"),
        };
        for event in events.iter().skip(1) {
            state.apply_event(event);
        }
        state
    }

    pub fn create(name: String, capacity: i16) -> Result<(Self, Event)> {
        Self::validate_capacity(capacity)?;
        let circle_id = CircleId::gen();
        let event_id = event_id::EventId::gen();

        let event = Event::new(
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

    // Private helper methods for event sourcing

    fn create_from_created_event(event: Event) -> Self {
        match event.data {
            event::EventData::CircleCreated(event::CircleCreated { name, capacity }) => Self {
                id: event.circle_id,
                name,
                capacity,
                version: event.version,
            },
            _ => panic!("Invalid event data"),
        }
    }

    fn apply_event(&mut self, event: &Event) {
        match &event.data {
            event::EventData::CircleCreated(event::CircleCreated { name, capacity }) => {
                self.name = name.clone();
                self.capacity = *capacity;
                self.version = event.version.clone();
            }
            event::EventData::CircleUpdated(event::CircleUpdated { name, capacity }) => {
                self.name = name.clone().unwrap_or(self.name.clone());
                self.capacity = capacity.unwrap_or(self.capacity);
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
