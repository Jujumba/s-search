use std::{error, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub(crate) enum TokenError {
    NonCharToken(char),
}
impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenError::NonCharToken(c) => write!(f, "non char token: {c}"),
        }
    }
}
impl error::Error for TokenError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    // todo: we may be interested in what is unexpected
    Unexpected,
    Useless,
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unexpected => write!(f, "unexpected"),
            Self::Useless => write!(f, "useless"),
        }
    }
}
impl error::Error for ParseError {}
