pub enum Error {
    OutOfGrade,
}

#[derive(Copy, Debug, PartialEq, Eq, Hash, Clone)]
pub enum Grade {
    First,
    Second,
    Third,
    Fourth,
}

impl std::convert::From<Grade> for i16 {
    fn from(value: Grade) -> Self {
        match value {
            Grade::First => 1,
            Grade::Second => 2,
            Grade::Third => 3,
            Grade::Fourth => 4,
        }
    }
}

impl std::convert::TryFrom<i16> for Grade {
    type Error = Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Grade::First,
            2 => Grade::Second,
            3 => Grade::Third,
            4 => Grade::Fourth,
            _ => return Err(Error::OutOfGrade),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        for (v, n) in [
            (Grade::First, 1),
            (Grade::Second, 2),
            (Grade::Third, 3),
            (Grade::Fourth, 4),
        ] {
            assert_eq!(i16::from(v), n);
            assert_eq!(
                Grade::try_from(n).map_err(|e| match e {
                    Error::OutOfGrade => anyhow::Error::msg("Out of grade"),
                })?,
                v
            );
        }
        Ok(())
    }
}
