use std::fmt::{Debug, Display};
use std::str::FromStr;

use super::lexer::{lex_one, Token, TokenKind};

#[derive(PartialEq)]
pub(crate) enum Node {
    Symbol(String),
    Number(u64),
    String(String),
    List(Vec<Node>),
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
            Node::Number(n) => write!(f, "{n}"),
            Node::String(s) => write!(f, "{s:?}"),
            Node::List(ns) => {
                f.write_str("[")?;
                let mut first = true;
                for n in ns {
                    if first {
                        first = false;
                    } else {
                        f.write_str(" ")?;
                    }
                    write!(f, "{n}")?;
                }
                f.write_str("]")
            }
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

pub(crate) enum ParseNodeError {
    Empty,
    Unexpected(char),
    Multiple,
    Other,
}

impl Display for ParseNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str("input is empty"),
            Self::Unexpected(c) => write!(f, "unexpected {c:?}"),
            Self::Multiple => f.write_str("multiple forms found"),
            Self::Other => f.write_str("other unsorted error"),
        }
    }
}

impl Debug for ParseNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

enum LexState {
    Empty,
    Full(Node),
    List(Vec<Node>, Box<LexState>),
}

pub(crate) fn lex(s: &str) -> Result<Node, ParseNodeError> {
    let s = s.as_bytes();
    let mut offset = 0;
    let mut state = LexState::Empty;

    while offset < s.len() {
        let Token { kind, excerpt } = lex_one(&s[offset..]);
        let consume = excerpt.len();
        if consume == 0 {
            return Err(ParseNodeError::Unexpected(s[offset] as char));
        }

        offset += consume;

        match state {
            LexState::Empty => match kind {
                TokenKind::Whitespace => {}
                TokenKind::Symbol => state = LexState::Full(Node::Symbol(excerpt.to_string())),
                TokenKind::Number => state = LexState::Full(Node::Number(excerpt.parse().unwrap())),
                TokenKind::String => state = LexState::Full(Node::String(excerpt.to_string())),
                TokenKind::ListStart => state = LexState::List(vec![], Box::new(state)),
                TokenKind::ListEnd => return Err(ParseNodeError::Unexpected(']')),
            },
            LexState::Full(_) => match kind {
                TokenKind::Whitespace => {}
                _ => return Err(ParseNodeError::Multiple),
            },
            LexState::List(ref mut ns, ref parent) => match kind {
                TokenKind::Whitespace => {}
                TokenKind::Symbol => ns.push(Node::Symbol(excerpt.to_string())),
                TokenKind::Number => ns.push(Node::Number(excerpt.parse().unwrap())),
                TokenKind::String => ns.push(Node::String(excerpt.to_string())),
                TokenKind::ListStart => state = LexState::List(vec![], Box::new(state)),
                TokenKind::ListEnd => match &**parent {
                    LexState::Empty => state = LexState::Full(Node::List(*ns)),
                    LexState::Full(_) => return Err(ParseNodeError::Multiple),
                    LexState::List(pns, pparent) => {
                        pns.push(Node::List(*ns));
                        state = LexState::List(*pns, &**pparent);
                    }
                },
            },
        }
    }

    match state {
        LexState::Empty => Err(ParseNodeError::Empty),
        LexState::Full(node) => Ok(node),
        _ => Err(ParseNodeError::Other),
    }
}
