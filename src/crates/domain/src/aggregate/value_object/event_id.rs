use std::fmt;

use rand::distr::{Alphanumeric, SampleString};

// use rand::distributions::{Alphanumeric, DistString};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventId(String);

impl EventId {
    pub fn gen() -> Self {
        let mut rng = rand::rng();
        Self(Alphanumeric.sample_string(&mut rng, 36))
    }
}

impl fmt::Display for EventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for EventId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl std::convert::From<i16> for EventId {
    fn from(id: i16) -> Self {
        Self(id.to_string())
    }
}

impl std::convert::From<EventId> for String {
    fn from(id: EventId) -> Self {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_gen() {
        let id = EventId::gen();
        assert_eq!(id.0.len(), 36);
    }

    #[test]
    fn test_from_i16() {
        let id = EventId::from(1);
        assert_eq!(id.to_string(), "1");
    }

    #[test]
    fn test_from_str() -> anyhow::Result<()> {
        let id = EventId::from_str("1")?;
        assert_eq!(id.to_string(), "1");
        Ok(())
    }

    #[test]
    fn test_from_event_id() {
        let id = EventId::from(1);
        let s: String = id.into();
        assert_eq!(s, "1");
    }

    #[test]
    fn test_display() {
        let id = EventId::from(1);
        assert_eq!(id.to_string(), "1");
    }
}
