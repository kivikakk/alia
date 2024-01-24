use num_traits::{FromBytes, FromPrimitive};
use std::{cell::RefCell, rc::Rc};

use super::{BuiltinVal, Module, Op, Val, Vm};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub(super) struct Pid(pub(super) usize);

pub(crate) struct Proc {
    pub(super) _pid: Pid,
    pub(super) module: Rc<RefCell<Module>>,
    pub(super) last: Option<Val>,
    code: Vec<u8>,
    ip: usize,
    stack: Vec<Val>,
}

impl Proc {
    pub(super) fn new(pid: Pid, module: Rc<RefCell<Module>>, code: Vec<u8>) -> Proc {
        Proc {
            _pid: pid,
            module,
            last: None,
            code,
            ip: 0,
            stack: vec![],
        }
    }

    pub(crate) fn step(&mut self, vm: &mut Vm) -> Step {
        let op = Op::from_u8(self.code[self.ip])
            .ok_or_else(|| format!("should be valid opcode, was {}", self.code[self.ip]))
            .unwrap();
        self.ip += 1;

        match op {
            Op::Nop => {}
            Op::ImmediateSymbolBare => {
                let slen = self.n::<usize>();
                let s = vm.interns.intern(&self.code[self.ip..self.ip + slen]);
                self.ip += slen;

                self.stack.push(Val::Symbol(None, s));
            }
            Op::ImmediateSymbolWithModule => {
                let mlen = self.n::<usize>();
                let m = vm.interns.intern(&self.code[self.ip..self.ip + mlen]);
                self.ip += mlen;

                let slen = self.n::<usize>();
                let s = vm.interns.intern(&self.code[self.ip..self.ip + slen]);
                self.ip += slen;

                self.stack.push(Val::Symbol(Some(m), s));
            }
            Op::ImmediateBooleanTrue => self.stack.push(Val::Boolean(true)),
            Op::ImmediateBooleanFalse => self.stack.push(Val::Boolean(false)),
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
            Op::Drop => {
                self.last = Some(self.stack.pop().expect("stack should not be empty"));
            }
            Op::Eval => {
                let form = self.stack.pop().expect("stack should not be empty");
                let result = self.eval(vm, &form);
                self.stack.push(result);
            }
            Op::Call => {
                let n = self.n::<usize>() - 1; // includes callee
                let args = self.stack.split_off(self.stack.len() - n);
                let callee = self.stack.pop().unwrap();
                let result = self.call(vm, &callee, &args);
                self.stack.push(result);
            }
            Op::JumpRelative => {
                let sip = self.ip;
                let n = self.n::<usize>();
                // So far this is a backwards jump only, relative to the opcode.
                self.ip = sip - n;
            }
        }

        if self.ip < self.code.len() {
            Step::Running
        } else {
            assert_eq!(0, self.stack.len(), "stack not empty at end");
            Step::Finished
        }
    }

    // TODO: consider Cow<Val> here or something?  Moooooooooooooo
    //                       _______________________________________
    //   (__) __ (  )       /                                       \
    //  _/  .   .   \_   --<  It may not be clear, but I am bovine.  >
    // ( |    w     | )     \                                       /
    //   ¯\________/ ¯       ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯
    //
    pub(super) fn eval(&mut self, vm: &mut Vm, form: &Val) -> Val {
        match form {
            &Val::Symbol(None, s) => {
                let self_module = self.module.borrow();
                match self_module.lookup(vm, s) {
                    Some(v) => v,
                    None => Val::Symbol(None, vm.interns.intern("panic TODO!".as_bytes())),
                }
            }
            &Val::Symbol(Some(m), s) => {
                let module = match vm.lookup_module(m) {
                    Some(v) => v,
                    None => return Val::Symbol(None, vm.interns.intern("panic TODO!".as_bytes())),
                };
                let module = module.borrow();
                match module.lookup(vm, s) {
                    Some(v) => v,
                    None => Val::Symbol(None, vm.interns.intern("panic TODO!".as_bytes())),
                }
            }
            Val::Boolean(_) | Val::Integer(_) | Val::Float(_) | Val::String(_) => {
                // primitives evaluate to themselves
                form.clone()
            }
            Val::List(ns) => {
                let head = match ns.len() {
                    0 => {
                        // empty cons evaluates to itself
                        return form.clone();
                    }
                    _ => &ns[0],
                };
                let callee = self.eval(vm, head);
                self.call(vm, &callee, &ns[1..])
            }
            Val::Vec(ns) => Val::Vec(ns.iter().map(|f| self.eval(vm, f)).collect::<Vec<_>>()),
            Val::Builtin(..) | Val::Module(..) => {
                // builtins and modules evaluate to themselves
                form.clone()
            }
        }
    }

    fn call(&mut self, vm: &mut Vm, callee: &Val, args: &[Val]) -> Val {
        match callee {
            Val::Builtin(BuiltinVal { code, .. }) => code(vm, self, args),
            _ => panic!("can't call {}", callee.format(vm)),
        }
    }

    fn n<T: FromBytes<Bytes = [u8; 8]>>(&mut self) -> T {
        let u = T::from_le_bytes(self.code[self.ip..self.ip + 8].try_into().unwrap());
        self.ip += 8;
        u
    }
}

pub(crate) enum Step {
    Running,
    Finished,
}
