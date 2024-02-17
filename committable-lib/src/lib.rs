use miette::{Diagnostic, SourceOffset, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic, PartialEq, Eq)]
#[error("{error_type:?}")]
#[diagnostic()]
pub struct SingleError {
    error_type: ErrorType,
    #[label("{error_type}")]
    bad_bit: SourceSpan,
}

// Define a global enum for error types
#[derive(Error, Debug, PartialEq, Eq)]
pub enum ErrorType {
    #[error("header is empty")]
    NonEmptyHeader,
    #[error("body is empty")]
    NonEmptyBody,
    #[error("there is not exactly one empty line before body")]
    SingleEmptyLineBeforeBody,
}

impl SingleError {
    fn new(error_type: ErrorType, start: SourceOffset, length: usize) -> Self {
        Self {
            error_type,
            bad_bit: SourceSpan::new(start, length),
        }
    }
}

#[derive(Debug, Error, Diagnostic, PartialEq, Eq)]
#[error("{} error(s)", others.len())]
pub struct GroupError {
    #[source_code]
    src: String,

    #[related]
    others: Vec<SingleError>,
}

pub struct Commit<'a> {
    commit_string: &'a str,
}

impl<'a> Commit<'a> {
    pub fn new(commit_string: &'a str) -> Self {
        Commit { commit_string }
    }

    pub fn get_commit_string(&self) -> &'a str {
        self.commit_string
    }

    pub fn get_header(&self) -> &'a str {
        self.commit_string.lines().next().unwrap_or("")
    }

    pub fn get_body(&self) -> Option<&'a str> {
        let Some((_, rest)) = self.commit_string.split_once('\n') else {
            return None;
        };

        let mut body = rest;
        while body.starts_with('\n') || body.starts_with("\r\n") {
            let Some((_, rest)) = body.split_once('\n') else {
                return None;
            };
            body = rest;
        }

        if body.is_empty() {
            None
        } else {
            Some(body)
        }
    }
}

fn check_commit(commit: &Commit, rules: &[Box<dyn Rule>]) -> Result<(), GroupError> {
    let commit_string = commit.get_commit_string();
    let mut errors = Vec::new();

    for rule in rules {
        if let Err(error) = rule.check(commit) {
            errors.push(error);
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(GroupError {
            src: commit_string.to_string(),
            others: errors,
        })
    }
}

pub fn check_all_rules(commit: &Commit) -> Result<(), GroupError> {
    check_commit(
        commit,
        &[
            Box::new(NonEmptyHeader {}),
            Box::new(NonEmptyBody {}),
            Box::new(SingleEmptyLineBeforeBody {}),
        ],
    )
}

trait Rule {
    fn check(&self, commit: &Commit) -> Result<(), SingleError>;
}

struct NonEmptyHeader {}
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

struct NonEmptyBody {}
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

struct SingleEmptyLineBeforeBody {}

impl Rule for SingleEmptyLineBeforeBody {
    fn check(&self, commit: &Commit) -> Result<(), SingleError> {
        let content = commit.commit_string;
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
    fn only_header() {
        let c = Commit::new("Hello world");
        assert_eq!(c.get_commit_string(), "Hello world");
        assert_eq!(c.get_header(), "Hello world");
        assert_eq!(c.get_body(), None);
    }

    #[test]
    fn empty_body() {
        let c = Commit::new("Hello world\n\n");
        assert_eq!(c.get_commit_string(), "Hello world\n\n");
        assert_eq!(c.get_header(), "Hello world");
        assert_eq!(c.get_body(), None);
    }

    #[test]
    fn some_body() {
        let c = Commit::new("Hello world\nSome Body\n");
        assert_eq!(c.get_commit_string(), "Hello world\nSome Body\n");
        assert_eq!(c.get_header(), "Hello world");
        assert_eq!(c.get_body(), Some("Some Body\n"));
    }

    #[test]
    fn more_body() {
        let c = Commit::new("Hello world\n\nSome Body");
        assert_eq!(c.get_commit_string(), "Hello world\n\nSome Body");
        assert_eq!(c.get_header(), "Hello world");
        assert_eq!(c.get_body(), Some("Some Body"));
    }

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
