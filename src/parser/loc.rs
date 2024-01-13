use std::fmt::{Debug, Display};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub(crate) struct Loc(pub(crate) usize, pub(crate) usize);

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Range(pub(crate) Loc, pub(crate) Loc);

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
