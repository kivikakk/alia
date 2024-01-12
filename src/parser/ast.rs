use std::fmt::{Debug, Display};
use std::str::{self, FromStr};

#[derive(PartialEq)]
pub(crate) enum Node {
    Symbol(String),
    Number(u64),
    String(String),
    List(Vec<Node>),
    Vec(Vec<Node>),
}

impl FromStr for Node {
    type Err = super::ParseNodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        super::parse(s)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Symbol(s) => f.write_str(s),
            Node::Number(n) => write!(f, "{n}"),
            Node::String(s) => write!(f, "{s:?}"),
            Node::List(ns) => {
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
            Node::Vec(ns) => {
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
