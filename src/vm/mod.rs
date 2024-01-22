mod builtins;
mod interns;
mod module;
mod ops;
mod proc;
mod val;

use std::cell::RefCell;
use std::collections::HashMap;
use std::mem;
use std::rc::Rc;

pub(crate) use self::ops::Op;

use self::interns::{InternedSymbol, Interns};
use self::module::Module;
use self::proc::{Pid, Proc, Step};
use self::val::Val;

pub(crate) struct Vm {
    modules: HashMap<InternedSymbol, RefCell<Rc<Module>>>,
    pub(super) interns: Interns,
    last_pid: Pid,
}

impl Vm {
    pub(crate) fn new() -> Self {
        let mut vm = Vm {
            modules: HashMap::new(),
            interns: Interns::new(),
            last_pid: Pid(0),
        };

        let builtins = Module::builtins(&mut vm);
        vm.modules.insert(
            vm.interns.intern("builtins"),
            RefCell::new(Rc::new(builtins)),
        );
        vm
    }

    pub(crate) fn run_to_completion(&mut self, code: Vec<u8>) -> Vec<Val> {
        let proc = self.schedule(code);
        self.step_to_end(proc)
    }

    fn schedule(&mut self, code: Vec<u8>) -> Proc {
        self.last_pid = Pid(self.last_pid.0 + 1);
        Proc::new(self.last_pid, code)
    }

    fn step_to_end(&mut self, mut proc: Proc) -> Vec<Val> {
        loop {
            match proc.step(self) {
                Step::Running => {}
                Step::Finished => {
                    return mem::take(&mut proc.stack);
                }
            }
        }
    }
}
