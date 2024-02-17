use crate::Commit;

use super::{ErrorType, Rule, SingleError};

pub struct BodyLength {}

const BODY_LEN: usize = 72;

impl Rule for BodyLength {
    fn check(&self, commit: &Commit) -> Result<(), SingleError> {
        let Some(mut body) = commit.get_body() else {
            return Ok(());
        };
        let Some(offset) = commit.body_offset() else {
            return Ok(());
        };
        println!("Offset: {offset}");

        let mut offset = offset;
        while let Some((line, rest)) = body.split_once('\n').or_else(|| {
            if !body.is_empty() {
                Some((body, ""))
            } else {
                None
            }
        }) {
            if line.len() > BODY_LEN {
                return Err(SingleError::new(
                    ErrorType::BodyLength,
                    (offset + BODY_LEN).into(),
                    line.len() - BODY_LEN,
                ));
            }
            offset += line.len() + 1;
            body = rest;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_length() {
        let c = BodyLength {};
        assert_eq!(
            c.check(&Commit::new(
                "\n\n012345678901234567890123456789012345678901234567890123456789012345678901\n\nFine!"
            )),
            Ok(())
        );
        assert_eq!(
            c.check(&Commit::new(
                "012345678901234567890123456789012345678901234567890123456789012345678901XX\n\nFine!"
            )),
            Ok(())
        );
        assert_eq!(
            c.check(&Commit::new(
                "\n\n012345678901234567890123456789012345678901234567890123456789012345678901XX"
            )),
            Err(SingleError::new(
                ErrorType::BodyLength,
                (2 + BODY_LEN).into(),
                2
            ))
        );
        assert_eq!(
            c.check(&Commit::new(
                "\n\n012345678901234567890123456789012345678901234567890123456789012345678901XX\n\nNotFine!"
            )),
            Err(SingleError::new(ErrorType::BodyLength, (2 + BODY_LEN).into(), 2))
        );
    }
}
