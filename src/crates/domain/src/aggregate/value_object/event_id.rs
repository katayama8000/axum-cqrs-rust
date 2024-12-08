use std::str::FromStr;

pub enum Error {
    Invalid,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventId(String);

impl EventId {
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
