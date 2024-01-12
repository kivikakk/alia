use std::fmt::{Debug, Display};
use std::str::{self, FromStr};

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
        parse(s)
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
    Unfinished,
    Unexpected(char),
    Multiple,
    Number,
    String,
    Other,
}

impl Display for ParseNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str("input is empty"),
            Self::Unfinished => f.write_str("input appears unfinished"),
            Self::Unexpected(c) => write!(f, "unexpected {c:?}"),
            Self::Multiple => f.write_str("multiple forms found"),
            Self::Number => f.write_str("number parse fail"),
            Self::String => f.write_str("string parse fail"),
            Self::Other => f.write_str("other unsorted error"),
        }
    }
}

impl Debug for ParseNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

enum ParseStackEntry {
    Empty,
    Full(Node),
    List(Vec<Node>),
}

struct ParseStack(Vec<ParseStackEntry>);

impl ParseStack {
    fn new() -> Self {
        ParseStack(vec![ParseStackEntry::Empty])
    }

    fn fill(&mut self, node: Node) -> Result<(), ParseNodeError> {
        let last = self.0.last_mut().unwrap();
        match last {
            ParseStackEntry::Empty => *last = ParseStackEntry::Full(node),
            ParseStackEntry::Full(_) => return Err(ParseNodeError::Multiple),
            ParseStackEntry::List(ref mut ns) => ns.push(node),
        }
        Ok(())
    }

    fn list_start(&mut self) -> Result<(), ParseNodeError> {
        let last = self.0.last_mut().unwrap();
        match last {
            ParseStackEntry::Empty | ParseStackEntry::List(_) => {
                self.0.push(ParseStackEntry::List(vec![]))
            }
            ParseStackEntry::Full(_) => return Err(ParseNodeError::Multiple),
        }
        Ok(())
    }

    fn list_end(&mut self) -> Result<(), ParseNodeError> {
        match self.0.pop().ok_or(ParseNodeError::Other)? {
            ParseStackEntry::List(ns) => Ok(self.fill(Node::List(ns))?),
            ParseStackEntry::Empty | ParseStackEntry::Full(_) => {
                Err(ParseNodeError::Unexpected(']'))
            }
        }
    }

    fn finish(mut self) -> Result<Node, ParseNodeError> {
        let first = self.0.pop().ok_or(ParseNodeError::Other)?;
        if !self.0.is_empty() {
            return Err(ParseNodeError::Unfinished);
        }
        match first {
            ParseStackEntry::Empty => Err(ParseNodeError::Empty),
            ParseStackEntry::Full(n) => Ok(n),
            _ => Err(ParseNodeError::Other),
        }
    }
}

pub(crate) fn parse(s: &str) -> Result<Node, ParseNodeError> {
    let s = s.as_bytes();
    let mut offset = 0;
    let mut stack = ParseStack::new();

    while offset < s.len() {
        let Token { kind, excerpt } = lex_one(&s[offset..]);
        let consume = excerpt.len();
        if consume == 0 {
            return Err(ParseNodeError::Unexpected(s[offset] as char));
        }

        offset += consume;

        match kind {
            TokenKind::Whitespace => {}
            TokenKind::Symbol => stack.fill(Node::Symbol(parse_symbol(excerpt)?))?,
            TokenKind::Number => stack.fill(Node::Number(parse_number(excerpt)?))?,
            TokenKind::String => stack.fill(Node::String(parse_string(excerpt)?))?,
            TokenKind::ListStart => stack.list_start()?,
            TokenKind::ListEnd => stack.list_end()?,
        }
    }

    stack.finish()
}

fn parse_symbol(s: &[u8]) -> Result<String, ParseNodeError> {
    Ok(unsafe { str::from_utf8_unchecked(s) }.to_string())
}

fn parse_number(s: &[u8]) -> Result<u64, ParseNodeError> {
    unsafe { str::from_utf8_unchecked(s) }
        .parse()
        .map_err(|_| ParseNodeError::Number)
}

fn parse_string(s: &[u8]) -> Result<String, ParseNodeError> {
    let len = s.len();
    let mut r = String::new();
    let mut i = 0;

    while i < len {
        match s[i] {
            b'\\' => {
                i += 1;
                if i == len {
                    return Err(ParseNodeError::String);
                }
                match s[i] {
                    b'\\' | b'"' => r.push(s[i] as char),
                    b't' => r.push('\t'),
                    b'r' => r.push('\r'),
                    b'n' => r.push('\n'),
                    _ => return Err(ParseNodeError::String),
                }
            }
            b'"' => {
                i += 1;
                if i == 1 {
                    continue;
                }
                if i == len {
                    return Ok(r);
                }
                // should really never happen
                return Err(ParseNodeError::String);
            }
            b => r.push(b as char),
        }

        i += 1;
    }

    Err(ParseNodeError::Unfinished)
}
