mod rules;

use miette::Diagnostic;
use rules::{
    non_empty_body::NonEmptyBody, non_empty_header::NonEmptyHeader,
    single_empty_line_before_body::SingleEmptyLineBeforeBody, Rule, SingleError,
};
use thiserror::Error;

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
}
