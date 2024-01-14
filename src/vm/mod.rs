mod interns;
mod ops;

use num_traits::{FromBytes, FromPrimitive};
use std::fmt::Write;
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
            let op = Op::from_u8(code[ip])
                .ok_or_else(|| format!("should be valid opcode, was {}", code[ip]))
                .unwrap();
            ip += 1;

            match op {
                Op::Nop => {}
                Op::ImmediateSymbol => {
                    let len = Self::n::<usize>(code, &mut ip);
                    let s = self.intern(&code[ip..ip + len]);
                    self.stack.push(Val::Symbol(s));
                    ip += len;
                }
                Op::ImmediateInteger => {
                    let i = Self::n::<i64>(code, &mut ip);
                    self.stack.push(Val::Integer(i));
                }
                Op::ImmediateFloat => {
                    let f = Self::n::<f64>(code, &mut ip);
                    self.stack.push(Val::Float(f));
                }
                Op::ImmediateString => {
                    let n = Self::n::<usize>(code, &mut ip);
                    let str = String::from_utf8(code[ip..ip + n].to_vec())
                        .expect("should be valid utf-8");
                    self.stack.push(Val::String(str));
                    ip += n;
                }
                Op::ConsList => {
                    let n = Self::n::<usize>(code, &mut ip);
                    let v = self.stack.split_off(self.stack.len() - n);
                    self.stack.push(Val::List(v));
                }
                Op::ConsVec => {
                    let n = Self::n::<usize>(code, &mut ip);
                    let v = self.stack.split_off(self.stack.len() - n);
                    self.stack.push(Val::Vec(v));
                }
                Op::Eval => {
                    let form = self.stack.pop().expect("stack should not be empty");
                    let result = self.eval(form);
                    self.stack.push(result);
                }
            }
        }

        mem::take(&mut self.stack)
    }

    fn eval(&mut self, form: Val) -> Val {
        match form {
            Val::Symbol(s) => {
                // TODO: resolve
                if s == self.interns("true") || s == self.interns("false") {
                    return form;
                }
                Val::Symbol(self.interns("nyonk"))
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

    fn n<T: FromBytes<Bytes = [u8; 8]>>(code: &[u8], ip: &mut usize) -> T {
        let u = T::from_le_bytes(code[*ip..*ip + 8].try_into().unwrap());
        *ip += 8;
        u
    }

    fn intern(&mut self, b: &[u8]) -> InternedSymbol {
        self.interns.intern(b)
    }
    fn interns(&mut self, s: &str) -> InternedSymbol {
        self.interns.intern(s.as_bytes())
    }
}

pub(crate) enum Val {
    Symbol(InternedSymbol),
    Integer(i64),
    Float(f64),
    String(String),
    List(Vec<Val>),
    Vec(Vec<Val>),
}

impl Val {
    pub(crate) fn format(&self, vm: &Vm) -> String {
        match self {
            Val::Symbol(i) => str::from_utf8(vm.interns.resolve(*i))
                .expect("all symbols should be utf-8")
                .to_string(),
            Val::Integer(i) => format!("{}", i),
            Val::Float(f) => format!("{}", f), // XXX
            Val::String(s) => s.to_string(),
            Val::List(ns) => {
                let mut s = "(".to_string();
                let mut first = true;
                for n in ns {
                    if first {
                        first = false;
                    } else {
                        write!(s, " ").unwrap();
                    }
                    write!(s, "{}", n.format(vm)).unwrap();
                }
                write!(s, ")").unwrap();
                s
            }
            Val::Vec(ns) => {
                let mut s = "[".to_string();
                let mut first = true;
                for n in ns {
                    if first {
                        first = false;
                    } else {
                        write!(s, " ").unwrap();
                    }
                    write!(s, "{}", n.format(vm)).unwrap();
                }
                write!(s, "]").unwrap();
                s
            }
        }
    }
}
