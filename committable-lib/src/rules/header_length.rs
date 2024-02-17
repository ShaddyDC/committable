use crate::Commit;

use super::{ErrorType, Rule, SingleError};

pub struct HeaderLength {}

const HEADER_LEN: usize = 50;

impl Rule for HeaderLength {
    fn check(&self, commit: &Commit) -> Result<(), SingleError> {
        let header = commit.get_header();

        if header.len() > HEADER_LEN {
            Err(SingleError::new(
                ErrorType::HeaderLength,
                HEADER_LEN.into(),
                header.len() - HEADER_LEN,
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
    fn header_length() {
        let c = HeaderLength {};
        assert_eq!(c.check(&Commit::new("H")), Ok(()));
        assert_eq!(
            c.check(&Commit::new(
                "01234567890123456789012345678901234567890123456789\n\nFine!"
            )),
            Ok(())
        );
        assert_eq!(
            c.check(&Commit::new(
                "01234567890123456789012345678901234567890123456789XX"
            )),
            Err(SingleError::new(
                ErrorType::HeaderLength,
                HEADER_LEN.into(),
                2
            ))
        );
        assert_eq!(
            c.check(&Commit::new(
                "01234567890123456789012345678901234567890123456789XX\n\nNotFine!"
            )),
            Err(SingleError::new(
                ErrorType::HeaderLength,
                HEADER_LEN.into(),
                2
            ))
        );
    }
}
