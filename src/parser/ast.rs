use std::fmt::{Debug, Display};
use std::str::{self, FromStr};

use super::{ParseError, ParseErrorKind};

#[derive(PartialEq)]
pub(crate) enum NodeValue {
    Symbol(String),
    Number(u64),
    String(String),
    List(Vec<Node>),
    Vec(Vec<Node>),
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub(crate) struct Loc(pub(crate) usize, pub(crate) usize);

#[derive(Clone, Copy, Debug)]
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

impl From<Loc> for lsp_types::Position {
    fn from(value: Loc) -> Self {
        Self::new(value.0 as u32, value.1 as u32)
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

impl From<Range> for lsp_types::Range {
    fn from(value: Range) -> Self {
        Self::new(value.0.into(), value.1.into())
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
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut doc = s.parse::<Document>()?;
        match doc.toplevels.len() {
            0 => Err(ParseError {
                kind: ParseErrorKind::Empty,
                range: doc.range,
            }),
            1 => Ok(doc.toplevels.pop().unwrap()),
            _ => Err(ParseError {
                kind: ParseErrorKind::Multiple,
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

pub(crate) struct Document {
    toplevels: Vec<Node>,
    range: Range,
}

impl Document {
    pub(crate) fn nodes_at<L: Into<Loc>>(&self, loc: L) -> Vec<&Node> {
        let mut nodes = vec![];
        let loc = loc.into();

        for toplevel in &self.toplevels {
            Self::nodes_at_recurse(toplevel, loc, &mut nodes);
        }

        nodes
    }

    fn nodes_at_recurse<'a>(node: &'a Node, loc: Loc, nodes: &mut Vec<&'a Node>) {
        if loc >= node.range.0 && loc < node.range.1 {
            nodes.push(node);
        }

        match &node.value {
            NodeValue::Symbol(..) | NodeValue::Number(..) | NodeValue::String(..) => {}
            NodeValue::List(ns) | NodeValue::Vec(ns) => {
                for n in ns {
                    Self::nodes_at_recurse(n, loc, nodes);
                }
            }
        }
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for toplevel in &self.toplevels {
            if first {
                first = false;
            } else {
                writeln!(f)?;
            }
            writeln!(f, "{toplevel}")?;
        }
        Ok(())
    }
}

impl Debug for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl FromStr for Document {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut toplevels = vec![];
        let mut offset = 0;
        let mut loc = Loc(0, 0);

        loop {
            match super::parse(s, offset, loc) {
                Ok((node, new_offset, new_loc)) => {
                    toplevels.push(node);
                    offset = new_offset;
                    loc = new_loc;
                }
                Err(ParseError {
                    kind: ParseErrorKind::Empty,
                    ..
                }) => break,
                Err(e) => return Err(e),
            }
        }

        Ok(Document {
            toplevels,
            range: ((0, 0).into(), loc).into(),
        })
    }
}
