use num_traits::{FromBytes, FromPrimitive};

use super::{interns, Interns, Op, Val};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub(super) struct Pid(pub(super) usize);

pub(super) struct Proc {
    // a reference to the vm? passed in on every call?
    // stack
    // context??
    pub(super) pid: Pid,
    code: Vec<u8>,
    ip: usize,
    pub(super) stack: Vec<Val>,
}

impl Proc {
    pub(super) fn new(pid: Pid, code: Vec<u8>) -> Proc {
        Proc {
            pid,
            code,
            ip: 0,
            stack: vec![],
        }
    }

    pub(crate) fn step(&mut self, interns: &mut Interns) -> bool {
        let op = Op::from_u8(self.code[self.ip])
            .ok_or_else(|| format!("should be valid opcode, was {}", self.code[self.ip]))
            .unwrap();
        self.ip += 1;

        match op {
            Op::Nop => {}
            Op::ImmediateSymbol => {
                let len = self.n::<usize>();
                let s = interns.intern(&self.code[self.ip..self.ip + len]);
                self.stack.push(Val::Symbol(s));
                self.ip += len;
            }
            Op::ImmediateInteger => {
                let i = self.n::<i64>();
                self.stack.push(Val::Integer(i));
            }
            Op::ImmediateFloat => {
                let f = self.n::<f64>();
                self.stack.push(Val::Float(f));
            }
            Op::ImmediateString => {
                let n = self.n::<usize>();
                let str = String::from_utf8(self.code[self.ip..self.ip + n].to_vec())
                    .expect("should be valid utf-8");
                self.stack.push(Val::String(str));
                self.ip += n;
            }
            Op::ConsList => {
                let n = self.n::<usize>();
                let v = self.stack.split_off(self.stack.len() - n);
                self.stack.push(Val::List(v));
            }
            Op::ConsVec => {
                let n = self.n::<usize>();
                let v = self.stack.split_off(self.stack.len() - n);
                self.stack.push(Val::Vec(v));
            }
            Op::Eval => {
                let form = self.stack.pop().expect("stack should not be empty");
                let result = self.eval(interns, form);
                self.stack.push(result);
            }
        }

        self.ip == self.code.len()
    }

    fn eval(&mut self, interns: &mut Interns, form: Val) -> Val {
        match form {
            Val::Symbol(s) => {
                // TODO: resolve
                if s == interns::TRUE || s == interns::FALSE {
                    return form;
                }
                Val::Symbol(interns.intern("nyonk".as_bytes()))
            }
            Val::Integer(_) | Val::Float(_) | Val::String(_) => {
                // primitives evaluate to themselves
                form
            }
            Val::List(ref _ns) => {
                // TODO: call
                form
            }
            Val::Vec(ref _ns) => {
                // TODO: cons
                form
            }
        }
    }

    fn n<T: FromBytes<Bytes = [u8; 8]>>(&mut self) -> T {
        let u = T::from_le_bytes(self.code[self.ip..self.ip + 8].try_into().unwrap());
        self.ip += 8;
        u
    }
}
