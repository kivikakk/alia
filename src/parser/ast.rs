use std::fmt::{Debug, Display};
use std::str::FromStr;

use super::lexer;

#[derive(PartialEq)]
pub(crate) enum Node {
    Symbol(String),
}

impl FromStr for Node {
    type Err = ParseNodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lex(s)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Symbol(s) => f.write_str(s),
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[derive(Debug)]
pub(crate) enum ParseNodeError {
    Empty,
    Unexpected(char),
    Multiple,
}

impl Display for ParseNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str("input is empty"),
            Self::Unexpected(c) => write!(f, "unexpected {c:?}"),
            Self::Multiple => f.write_str("multiple forms found"),
        }
    }
}

pub(crate) fn lex(s: &str) -> Result<Node, ParseNodeError> {
    let s = s.as_bytes();
    let mut offset = 0;
    let mut result: Option<Node> = None;

    while offset < s.len() {
        match lexer::lex_one(&s[offset..]) {
            Some(lexer::LexOne { consume, node }) => {
                assert!(consume > 0);
                offset += consume;
                if let Some(node) = node {
                    if result.is_some() {
                        return Err(ParseNodeError::Multiple);
                    }
                    result = Some(node);
                }
            }
            None => return Err(ParseNodeError::Unexpected(s[offset] as char)),
        }
    }

    if let Some(node) = result {
        Ok(node)
    } else {
        Err(ParseNodeError::Empty)
    }
}
