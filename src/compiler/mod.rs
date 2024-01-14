mod error;

pub(crate) use error::Error;
use num_traits::ToBytes;

use crate::parser::{self, Document, Node, NodeValue};
use crate::vm::Op;

pub(crate) fn compile(s: &str) -> Result<Vec<u8>, parser::Error> {
    let doc: Document = s.parse()?;

    Ok(vec![])
}

pub(crate) fn compile_one(n: Node) -> Result<Vec<u8>, Error> {
    let mut c = Compiler::new();
    c.node(n)?;
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

    fn node(&mut self, n: Node) -> Result<(), Error> {
        match &n.value {
            NodeValue::Symbol(s) => {
                self.op(Op::ImmediateSymbol)?;
                self.bytes(&s)?;
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
                self.bytes(&s)?;
                Ok(())
            }
            NodeValue::List(ns) => todo!(),
            NodeValue::Vec(ns) => todo!(),
        }
    }
}
