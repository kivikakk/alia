use std::fmt::{Debug, Display};
use std::str::{self, FromStr};

#[derive(PartialEq)]
pub(crate) enum NodeValue {
    Symbol(String),
    Number(u64),
    String(String),
    List(Vec<Node>),
    Vec(Vec<Node>),
}

#[derive(Clone, Copy)]
pub(crate) struct Loc(pub(crate) usize, pub(crate) usize);

#[derive(Clone, Copy)]
pub(crate) struct Range(pub(crate) Loc, pub(crate) Loc);

impl Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}

impl From<(usize, usize)> for Loc {
    fn from(value: (usize, usize)) -> Self {
        Loc(value.0, value.1)
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}

impl From<(Loc, Loc)> for Range {
    fn from(value: (Loc, Loc)) -> Self {
        Range(value.0, value.1)
    }
}

impl From<((usize, usize), (usize, usize))> for Range {
    fn from(value: ((usize, usize), (usize, usize))) -> Self {
        Range(value.0.into(), value.1.into())
    }
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
    type Err = super::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        super::parse(s, (0, 0))
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
