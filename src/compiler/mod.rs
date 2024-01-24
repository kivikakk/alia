mod error;
mod tests;

pub(crate) use error::Error;
use num_traits::ToBytes;

use crate::parser::{Node, NodeValue};
use crate::vm::Op;

pub(crate) fn compile_one(n: &Node) -> Result<Vec<u8>, Error> {
    let mut c = Compiler::new();
    c.node(n)?;
    c.eval()?;
    Ok(c.out)
}

struct Compiler {
    out: Vec<u8>,
}

impl Compiler {
    fn new() -> Self {
        Compiler { out: vec![] }
    }

    fn op(&mut self, op: Op) -> Result<(), Error> {
        self.out.push(op as u8);
        Ok(())
    }

    fn n<T: ToBytes<Bytes = [u8; 8]>>(&mut self, u: T) -> Result<(), Error> {
        self.out.extend_from_slice(&u.to_le_bytes());
        Ok(())
    }

    fn bytes<S: AsRef<[u8]>>(&mut self, s: S) -> Result<(), Error> {
        let s = s.as_ref();
        self.n(s.len())?;
        self.out.extend_from_slice(s);
        Ok(())
    }

    fn node(&mut self, n: &Node) -> Result<(), Error> {
        match &n.value {
            NodeValue::Symbol(None, s) => {
                self.op(Op::ImmediateSymbolBare)?;
                self.bytes(s)?;
                Ok(())
            }
            NodeValue::Symbol(Some(m), s) => {
                self.op(Op::ImmediateSymbolWithModule)?;
                self.bytes(m)?;
                self.bytes(s)?;
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
                for n in ns {
                    self.node(n)?;
                }
                self.op(Op::ConsList)?;
                self.n(ns.len())?;
                Ok(())
            }
            NodeValue::Vec(ns) => {
                for n in ns {
                    self.node(n)?;
                }
                self.op(Op::ConsVec)?;
                self.n(ns.len())?;
                Ok(())
            }
        }
    }

    fn eval(&mut self) -> Result<(), Error> {
        self.op(Op::Eval)?;
        Ok(())
    }
}
