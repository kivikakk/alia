mod interns;
mod ops;

use num_traits::FromPrimitive;
use std::{mem, str};

use self::interns::{InternedSymbol, Interns};
pub(crate) use ops::Op;

pub(crate) struct Vm {
    stack: Vec<Val>,
    interns: Interns,
}

impl Vm {
    pub(crate) fn new() -> Self {
        Vm {
            stack: vec![],
            interns: Interns::new(),
        }
    }

    pub(crate) fn exec(&mut self, code: &[u8]) -> Vec<Val> {
        let mut ip = 0;

        while ip < code.len() {
            let op = Op::from_u8(code[ip]).expect("should be valid opcode");
            ip += 1;

            match op {
                Op::Nop => {}
                Op::ImmediateSymbol => {
                    let len = Self::usize(code, &mut ip);
                    self.intern(&code[ip..ip + len]);
                    ip += len;
                }
                _ => todo!(),
            }
            ip += 1;
        }

        mem::take(&mut self.stack)
    }

    fn usize(code: &[u8], ip: &mut usize) -> usize {
        let u = usize::from_le_bytes(code[*ip..*ip + 8].try_into().unwrap());
        *ip += 8;
        u
    }

    fn intern(&mut self, b: &[u8]) {
        self.stack.push(Val::Symbol(self.interns.intern(b)))
    }
}

pub(crate) enum Val {
    Symbol(InternedSymbol),
}

impl Val {
    pub(crate) fn format(&self, vm: &Vm) -> String {
        match self {
            Val::Symbol(i) => str::from_utf8(vm.interns.resolve(*i))
                .expect("all symbols should be utf-8")
                .to_string(),
        }
    }
}
