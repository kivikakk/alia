mod error;
mod tests;

use num_traits::ToBytes;
use std::mem;

pub(crate) use self::error::Error;
use crate::parser::{Document, Node, NodeValue};
use crate::vm::Op;

pub(crate) struct Compiler {
    out: Vec<u8>,
}

type Result = core::result::Result<(), Error>;

impl Compiler {
    pub(crate) fn new() -> Self {
        Compiler { out: vec![] }
    }

    pub(crate) fn finish(&mut self) -> Vec<u8> {
        mem::take(&mut self.out)
    }

    pub(crate) fn doc(&mut self, doc: &Document) -> Result {
        for toplevel in &doc.toplevels {
            self.toplevel(toplevel)?;
        }
        Ok(())
    }

    fn toplevel(&mut self, n: &Node) -> Result {
        match n.value {
            NodeValue::Symbol(..)
            | NodeValue::Integer(_)
            | NodeValue::Float(_)
            | NodeValue::String(_)
            | NodeValue::Vec(_) => {
                // warn: side-effects only
                // (and int/float/string can't even do that).
                self.expr(n)?;
                self.op(Op::Drop)?;
            }
            NodeValue::List(_) => todo!(),
        }
        Ok(())
    }

    fn expr(&mut self, n: &Node) -> Result {
        match &n.value {
            NodeValue::Symbol(None, s) => {
                // TODO: proper compile-time resolution! not this shit!
                // XXX: local binds are ignored
                if s == "true" {
                    self.op(Op::ImmediateBooleanTrue)
                } else if s == "false" {
                    self.op(Op::ImmediateBooleanFalse)
                } else {
                    self.op(Op::ImmediateSymbolBare)?;
                    self.bytes(s)?;
                    self.op(Op::Eval)?;
                    Ok(())
                }
            }
            NodeValue::Symbol(Some(m), s) => {
                self.op(Op::ImmediateSymbolWithModule)?;
                self.bytes(m)?;
                self.bytes(s)?;
                self.op(Op::Eval)?;
                Ok(())
            }
            NodeValue::Integer(i) => {
                self.op(Op::ImmediateInteger)?;
                self.n(*i)?;
                Ok(())
            }
            NodeValue::Float(f) => {
                self.op(Op::ImmediateFloat)?;
                self.n(*f)?;
                Ok(())
            }
            NodeValue::String(s) => {
                self.op(Op::ImmediateString)?;
                self.bytes(s)?;
                Ok(())
            }
            NodeValue::List(ns) => {
                let mut ns = ns.iter();
                let head = ns.next().expect("list should have a head");
                self.expr(head)?; // <- resolves
                let mut i: usize = 0;
                for n in ns {
                    // XXX: causing eager evaluation
                    self.expr(n)?;
                    i += 1;
                }
                self.op(Op::Call)?;
                self.n(i)?;
                Ok(())
            }
            NodeValue::Vec(ns) => {
                for n in ns {
                    self.expr(n)?;
                }
                self.op(Op::ConsVec)?;
                self.n(ns.len())?;
                Ok(())
            }
        }
    }

    fn op(&mut self, op: Op) -> Result {
        self.out.push(op as u8);
        Ok(())
    }

    fn n<T: ToBytes<Bytes = [u8; 8]>>(&mut self, u: T) -> Result {
        self.out.extend_from_slice(&u.to_le_bytes());
        Ok(())
    }

    fn bytes<S: AsRef<[u8]>>(&mut self, s: S) -> Result {
        let s = s.as_ref();
        self.n(s.len())?;
        self.out.extend_from_slice(s);
        Ok(())
    }
}
