use std::fmt::{Debug, Display};

use super::Range;

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

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.value, f)
    }
}

#[derive(PartialEq)]
pub(crate) enum NodeValue {
    Symbol(Option<String>, String),
    Integer(i64),
    Float(f64),
    String(String),
    List(Vec<Node>),
    Vec(Vec<Node>),
}

impl Display for NodeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeValue::Symbol(None, s) => f.write_str(s),
            NodeValue::Symbol(Some(m), s) => write!(f, "{m}/{s}"),
            NodeValue::Integer(i) => write!(f, "{i}"),
            NodeValue::Float(d) => write!(f, "{d:?}"),
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
