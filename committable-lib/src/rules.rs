use miette::{Diagnostic, SourceOffset, SourceSpan};
use thiserror::Error;

use crate::Commit;

pub mod body_length;
pub mod header_length;
pub mod non_empty_body;
pub mod non_empty_header;
pub mod single_empty_line_before_body;

pub trait Rule {
    fn check(&self, commit: &Commit) -> Result<(), SingleError>;
}

#[derive(Debug, Error, Diagnostic, PartialEq, Eq)]
#[error("{error_type:?}")]
#[diagnostic()]
pub struct SingleError {
    error_type: ErrorType,
    #[label("{error_type}")]
    bad_bit: SourceSpan,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ErrorType {
    #[error("header is empty")]
    NonEmptyHeader,
    #[error("body is empty")]
    NonEmptyBody,
    #[error("there is not exactly one empty line before body")]
    SingleEmptyLineBeforeBody,
    #[error("header is too long")]
    HeaderLength,
    #[error("body line is too long")]
    BodyLength,
}

impl SingleError {
    fn new(error_type: ErrorType, start: SourceOffset, length: usize) -> Self {
        Self {
            error_type,
            bad_bit: SourceSpan::new(start, length),
        }
    }
}
