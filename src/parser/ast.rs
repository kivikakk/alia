use std::{fmt::Display, str::FromStr};

use super::lexer;

pub(crate) enum Node {
    Symbol(String),
}

#[derive(Debug)]
pub(crate) struct ParseNodeError;

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
                    assert!(result.is_none());
                    result = Some(node);
                }
            }
            None => return Err(ParseNodeError),
        }
    }

    if let Some(node) = result {
        Ok(node)
    } else {
        Err(ParseNodeError)
    }
}
