use rand::distributions::{Alphanumeric, DistString};
use std::str::FromStr;

pub enum Error {
    Invalid,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventId(String);

impl EventId {
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        Self(Alphanumeric.sample_string(&mut rng, 36))
    }
    pub fn new(id: String) -> Self {
        EventId(id)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl FromStr for EventId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(EventId(s.to_string()))
    }
}
