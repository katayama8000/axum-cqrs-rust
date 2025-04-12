use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    OutOfRange,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::OutOfRange => write!(f, "Value is out of range"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Version(u32);

impl Version {
    pub fn new() -> Self {
        Self(1)
    }

    pub fn next(&self) -> Self {
        if self.0 == u32::MAX {
            panic!("Version overflow");
        }
        Self(self.0 + 1)
    }
}

// Conversions between Version and other types
impl From<Version> for u32 {
    fn from(version: Version) -> u32 {
        version.0
    }
}

impl From<u32> for Version {
    fn from(n: u32) -> Self {
        Self(n)
    }
}

impl From<Version> for i64 {
    fn from(version: Version) -> i64 {
        version.0 as i64
    }
}

impl TryFrom<i64> for Version {
    type Error = Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        u32::try_from(value)
            .map(Self)
            .map_err(|_| Error::OutOfRange)
    }
}

impl TryFrom<i32> for Version {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(Error::OutOfRange);
        }
        u32::try_from(value as u32)
            .map(Self)
            .map_err(|_| Error::OutOfRange)
    }
}

impl TryFrom<Version> for i32 {
    type Error = Error;

    fn try_from(version: Version) -> Result<Self, Self::Error> {
        if version.0 > i32::MAX as u32 {
            return Err(Error::OutOfRange);
        }
        Ok(version.0 as i32)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(any(test, feature = "test-utils"))]
impl rand::distributions::Distribution<Version> for rand::distributions::Standard {
    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> Version {
        Version(rng.gen())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_initialization() {
        let v1 = Version::new();
        assert_eq!(u32::from(v1), 1);
        assert_eq!(u32::from(v1.next()), 2);
    }

    #[test]
    fn test_conversion_to_and_from_i64() -> anyhow::Result<()> {
        let max = i64::from(u32::MAX);
        let min = 0_i64;

        assert!(
            Version::try_from(max + 1).is_err(),
            "Out-of-range should fail"
        );
        assert!(
            Version::try_from(min - 1).is_err(),
            "Negative value should fail"
        );
        assert!(Version::try_from(max).is_ok());
        assert!(Version::try_from(min).is_ok());

        let version_max = Version::try_from(max).map_err(|_| anyhow::anyhow!("max"))?;
        let version_min = Version::try_from(min).map_err(|_| anyhow::anyhow!("min"))?;

        assert_eq!(i64::from(version_max), max);
        assert_eq!(i64::from(version_min), min);

        Ok(())
    }
}
