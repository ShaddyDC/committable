use crate::Commit;

use super::{ErrorType, Rule, SingleError};

pub struct SingleEmptyLineBeforeBody {}

impl Rule for SingleEmptyLineBeforeBody {
    fn check(&self, commit: &Commit) -> Result<(), SingleError> {
        let content = commit.get_commit_string();
        let Some((header, body)) = content.split_once('\n') else {
            return Ok(());
        };

        if !body.is_empty() && !body.starts_with('\n') && !body.starts_with("\r\n") {
            let line_len = body.find('\n').unwrap_or(body.len());
            return Err(SingleError::new(
                ErrorType::SingleEmptyLineBeforeBody,
                (header.len() + 1).into(),
                line_len,
            ));
        }

        let (first_line, rest) = body.split_once('\n').unwrap_or(("", body));

        if rest.starts_with('\n') || rest.starts_with("\r\n") {
            let Some(len) = rest.find(|c| c != '\n' && c != '\r') else {
                // No body, so skip warning
                return Ok(());
            };
            return Err(SingleError::new(
                ErrorType::SingleEmptyLineBeforeBody,
                (header.len() + 1 + first_line.len()).into(),
                len,
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_empty_line_before_body() {
        let c = SingleEmptyLineBeforeBody {};
        assert_eq!(c.check(&Commit::new("H")), Ok(()));
        assert_eq!(c.check(&Commit::new("H\n")), Ok(()));
        assert_eq!(c.check(&Commit::new("H\n\nTest")), Ok(()));
        assert_eq!(
            c.check(&Commit::new("H\n\n\nTest")),
            Err(SingleError::new(
                ErrorType::SingleEmptyLineBeforeBody,
                2.into(),
                1
            ))
        );
        assert_eq!(
            c.check(&Commit::new("H\nYEP\n\nTest")),
            Err(SingleError::new(
                ErrorType::SingleEmptyLineBeforeBody,
                2.into(),
                3
            ))
        );
        assert_eq!(
            c.check(&Commit::new("H\nYEP")),
            Err(SingleError::new(
                ErrorType::SingleEmptyLineBeforeBody,
                2.into(),
                3
            ))
        );
        assert_eq!(c.check(&Commit::new("H\n\n\n")), Ok(()));
        assert_eq!(c.check(&Commit::new("H\n\nNice!\n\n")), Ok(()));
    }
}
