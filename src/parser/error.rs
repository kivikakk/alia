use std::fmt::{Debug, Display};

use super::Range;

pub(crate) struct Error {
    pub(crate) kind: ErrorKind,
    pub(crate) range: Range,
}

impl Error {
    pub(super) fn new<R: Into<Range>>(kind: ErrorKind, range: R) -> Self {
        Error {
            kind,
            range: range.into(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at [{}]", self.kind, self.range)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

pub(crate) enum ErrorKind {
    Empty,
    Unfinished,
    Unexpected(char),
    Multiple,
    Number,
    String,
    Symbol,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str("input is empty"),
            Self::Unfinished => f.write_str("input appears unfinished"),
            Self::Unexpected(c) => write!(f, "unexpected {c:?}"),
            Self::Multiple => f.write_str("multiple forms found"),
            Self::Number => f.write_str("number parse fail"),
            Self::String => f.write_str("string parse fail"),
            Self::Symbol => f.write_str("symbol parse fail"),
        }
    }
}
