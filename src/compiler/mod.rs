mod error;
mod tests;

use num_traits::ToBytes;
use std::mem;

pub(crate) use self::error::Error;
use crate::parser::{Document, Node, NodeValue};
use crate::vm::Op;

macro_rules! guard {
    ($self:ident.$lhs:tt = $rhs:expr; $body:tt) => {
        let old_lhs = $self.$lhs;
        $self.$lhs = $rhs;
        $body;
        $self.$lhs = old_lhs;
    };
}

pub(crate) struct Compiler {
    out: Vec<u8>,
    omit_evals: bool,
}

impl Compiler {
    pub(crate) fn new() -> Self {
        Compiler {
            out: vec![],
            omit_evals: false,
        }
    }

    pub(crate) fn finish(&mut self) -> Vec<u8> {
        mem::take(&mut self.out)
    }

    pub(crate) fn doc(&mut self, doc: &Document) {
        for toplevel in &doc.toplevels {
            self.toplevel(toplevel);
        }
    }

    fn toplevel(&mut self, n: &Node) {
        match n.value {
            NodeValue::Symbol(..)
            | NodeValue::Integer(_)
            | NodeValue::Float(_)
            | NodeValue::String(_)
            | NodeValue::Vec(_) => {
                // warn: side-effects only
                // (and int/float/string can't even do that).
                self.expr(n);
                self.op(Op::Drop);
            }
            NodeValue::List(_) => {
                // XXX for now, side-effects only
                self.expr(n);
                self.op(Op::Drop);
            }
        }
    }

    fn expr(&mut self, n: &Node) {
        match &n.value {
            NodeValue::Symbol(None, s) => {
                // TODO: proper compile-time resolution! not this shit!
                // XXX: local binds are ignored
                if s == "true" {
                    self.op(Op::ImmediateBooleanTrue)
                } else if s == "false" {
                    self.op(Op::ImmediateBooleanFalse)
                } else {
                    self.op(Op::ImmediateSymbolBare);
                    self.bytes(s);
                    self.op(Op::Eval);
                }
            }
            NodeValue::Symbol(Some(m), s) => {
                self.op(Op::ImmediateSymbolWithModule);
                self.bytes(m);
                self.bytes(s);
                self.op(Op::Eval);
            }
            NodeValue::Integer(i) => {
                self.op(Op::ImmediateInteger);
                self.n(*i);
            }
            NodeValue::Float(f) => {
                self.op(Op::ImmediateFloat);
                self.n(*f);
            }
            NodeValue::String(s) => {
                self.op(Op::ImmediateString);
                self.bytes(s);
            }
            NodeValue::List(ns) => {
                let mut ns = ns.iter();
                let head = ns.next().expect("list should have a head");
                self.expr(head); // <- resolves
                let mut i: usize = 1;
                guard!(self.omit_evals = true; ({
                    for n in ns {
                        self.expr(n);
                        i += 1;
                    }
                }));
                self.op(Op::Call);
                self.n(i);
            }
            NodeValue::Vec(ns) => {
                for n in ns {
                    self.expr(n);
                }
                self.op(Op::ConsVec);
                self.n(ns.len());
            }
        }
    }

    fn op(&mut self, op: Op) {
        match (&op, self.omit_evals) {
            (&Op::Eval, true) => {}
            (&Op::Call, true) => self.out.push(Op::ConsList as u8),
            _ => self.out.push(op as u8),
        }
    }

    fn n<T: ToBytes<Bytes = [u8; 8]>>(&mut self, u: T) {
        self.out.extend_from_slice(&u.to_le_bytes());
    }

    fn bytes<S: AsRef<[u8]>>(&mut self, s: S) {
        let s = s.as_ref();
        self.n(s.len());
        self.out.extend_from_slice(s);
    }
}
