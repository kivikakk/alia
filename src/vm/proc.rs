use num_traits::{FromBytes, FromPrimitive};

use super::{interns, Op, Val, Vm};

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

    pub(crate) fn step(&mut self, vm: &mut Vm) -> Step {
        let op = Op::from_u8(self.code[self.ip])
            .ok_or_else(|| format!("should be valid opcode, was {}", self.code[self.ip]))
            .unwrap();
        self.ip += 1;

        match op {
            Op::Nop => {}
            Op::ImmediateSymbol => {
                let len = self.n::<usize>();
                let s = vm.interns.intern(&self.code[self.ip..self.ip + len]);
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
                let result = self.eval(vm, &form);
                self.stack.push(result);
            }
        }

        if self.ip < self.code.len() {
            Step::Running
        } else {
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
    fn eval(&mut self, vm: &mut Vm, form: &Val) -> Val {
        match form {
            &Val::Symbol(s) => {
                // `true`/`false` evaluate to themselves
                if s == interns::TRUE || s == interns::FALSE {
                    return form.clone();
                }
                // TODO/RESUME:
                // We want to look up e.g. builtins/print.
                // Right now this is one whole symbol, but interning the ns alongside
                // makes no sense at all, especially since we might use altered names
                // in places etc. etc.  a/x, .y/x, a.y.z/x are all possible.
                // Separate the ns components from the rest, then we can do a lookup
                // here without doing string munging in the VM (!).
                match vm.modules.get(&s) {
                    Some(v) => Val::Module(v.clone()),
                    None => Val::Symbol(vm.interns.intern("TODO!".as_bytes())),
                }
            }
            Val::Integer(_) | Val::Float(_) | Val::String(_) => {
                // primitives evaluate to themselves
                form.clone()
            }
            Val::List(ref ns) => {
                // empty cons evaluates to itself
                if ns.is_empty() {
                    return form.clone();
                }
                // TODO: call
                form.clone()
            }
            Val::Vec(ref ns) => {
                Val::Vec(ns.into_iter().map(|f| self.eval(vm, f)).collect::<Vec<_>>())
            }
            Val::Builtin(..) | Val::Module(..) => {
                // builtins and modules evaluate to themselves
                form.clone()
            }
        }
    }

    fn n<T: FromBytes<Bytes = [u8; 8]>>(&mut self) -> T {
        let u = T::from_le_bytes(self.code[self.ip..self.ip + 8].try_into().unwrap());
        self.ip += 8;
        u
    }
}

pub(super) enum Step {
    Running,
    Finished,
}
