mod commit;
mod rules;

use commit::Commit;
use miette::Diagnostic;
use rules::{
    body_length::BodyLength, header_length::HeaderLength, non_empty_body::NonEmptyBody,
    non_empty_header::NonEmptyHeader, single_empty_line_before_body::SingleEmptyLineBeforeBody,
    Rule, SingleError,
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
            Box::new(HeaderLength {}),
            Box::new(BodyLength {}),
        ],
    )
}
