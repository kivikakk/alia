use std::fmt::{Debug, Display};
use std::str::{self, FromStr};

use super::{Document, Range};
use crate::parser;

#[derive(PartialEq)]
pub(crate) enum NodeValue {
    Symbol(String),
    Number(u64),
    String(String),
    List(Vec<Node>),
    Vec(Vec<Node>),
}

pub(crate) struct Node {
    pub(crate) value: NodeValue,
    pub(crate) range: Range,
}

impl Node {
    pub(crate) fn new<R: Into<Range>>(value: NodeValue, range: R) -> Self {
        Node {
            value,
            range: range.into(),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl FromStr for Node {
    type Err = parser::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut doc = s.parse::<Document>()?;
        match doc.toplevels.len() {
            0 => Err(parser::Error {
                kind: parser::ErrorKind::Empty,
                range: doc.range,
            }),
            1 => Ok(doc.toplevels.pop().unwrap()),
            _ => Err(parser::Error {
                kind: parser::ErrorKind::Multiple,
                range: doc.range,
            }),
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.value, f)
    }
}

impl Display for NodeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeValue::Symbol(s) => f.write_str(s),
            NodeValue::Number(n) => write!(f, "{n}"),
            NodeValue::String(s) => write!(f, "{s:?}"),
            NodeValue::List(ns) => {
                f.write_str("(")?;
                let mut first = true;
                for n in ns {
                    if first {
                        first = false;
                    } else {
                        f.write_str(" ")?;
                    }
                    write!(f, "{n}")?;
                }
                f.write_str(")")
            }
            NodeValue::Vec(ns) => {
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

impl Debug for NodeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
