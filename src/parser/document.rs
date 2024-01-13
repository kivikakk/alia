use std::fmt::{Debug, Display};
use std::str::{self, FromStr};

use super::{Loc, Node, NodeValue, Range};
use crate::parser;

pub(crate) struct Document {
    pub(crate) toplevels: Vec<Node>,
    pub(crate) range: Range,
}

impl Document {
    pub(crate) fn nodes_at<L: Into<Loc>>(&self, loc: L) -> Vec<&Node> {
        let mut nodes = vec![];
        let loc = loc.into();

        for toplevel in &self.toplevels {
            Self::nodes_at_recurse(toplevel, loc, &mut nodes);
        }

        nodes.reverse();
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

impl PartialEq for Document {
    fn eq(&self, other: &Self) -> bool {
        self.toplevels == other.toplevels
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
    type Err = parser::Error;

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
                Err(parser::Error {
                    kind: parser::ErrorKind::Empty,
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
