use crate::Commit;

use super::{ErrorType, Rule, SingleError};

pub struct NonEmptyHeader {}

impl Rule for NonEmptyHeader {
    fn check(&self, commit: &Commit) -> Result<(), SingleError> {
        if commit.get_header().trim().is_empty() {
            Err(SingleError::new(
                ErrorType::NonEmptyHeader,
                0.into(),
                commit.get_header().len(),
            ))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nonempty_header() {
        let c = NonEmptyHeader {};
        assert_eq!(
            c.check(&Commit::new("")),
            Err(SingleError::new(ErrorType::NonEmptyHeader, 0.into(), 0))
        );
        assert_eq!(
            c.check(&Commit::new("\nTest")),
            Err(SingleError::new(ErrorType::NonEmptyHeader, 0.into(), 0))
        );
        assert_eq!(
            c.check(&Commit::new("\n\nTest")),
            Err(SingleError::new(ErrorType::NonEmptyHeader, 0.into(), 0))
        );
        assert_eq!(
            c.check(&Commit::new("  \nTest")),
            Err(SingleError::new(ErrorType::NonEmptyHeader, 0.into(), 2))
        );
        assert_eq!(c.check(&Commit::new("YEP\n\nTest")), Ok(()));
        assert_eq!(c.check(&Commit::new("YEP")), Ok(()));
    }
}
