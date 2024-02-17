use crate::Commit;

use super::{ErrorType, Rule, SingleError};

pub struct NonEmptyBody {}

impl Rule for NonEmptyBody {
    fn check(&self, commit: &Commit) -> Result<(), SingleError> {
        let message = commit.get_commit_string();
        let Some((header, body)) = message.split_once('\n') else {
            return Ok(());
        };

        if !body.is_empty() && commit.get_body().is_none() {
            Err(SingleError::new(
                ErrorType::NonEmptyBody,
                (header.len() + 1).into(),
                body.len(),
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
    fn nonempty_body() {
        let c = NonEmptyBody {};
        assert_eq!(c.check(&Commit::new("H")), Ok(()));
        assert_eq!(c.check(&Commit::new("H\n")), Ok(()));
        assert_eq!(c.check(&Commit::new("H\n\nTest")), Ok(()));
        assert_eq!(c.check(&Commit::new("H\n\n\nTest")), Ok(()));
        assert_eq!(c.check(&Commit::new("H\nYEP\n\nTest")), Ok(()));
        assert_eq!(c.check(&Commit::new("H\nYEP")), Ok(()));
        assert_eq!(
            c.check(&Commit::new("H\n\n")),
            Err(SingleError::new(ErrorType::NonEmptyBody, 2.into(), 1))
        );
    }
}
