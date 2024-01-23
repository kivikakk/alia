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

    pub(crate) fn anonymous_module(&mut self, name: &str) -> RefCell<Rc<Module>> {
        let mut module = Module::new(name.to_string());
        module.refer(
            self.modules
                .get(&self.interns.intern("builtins"))
                .unwrap()
                .clone(),
        );
        RefCell::new(Rc::new(module))
    }

    pub(crate) fn run_to_completion(
        &mut self,
        module: RefCell<Rc<Module>>,
        code: Vec<u8>,
    ) -> Vec<Val> {
        let proc = self.schedule(module, code);
        self.step_to_end(proc)
    }

    pub(crate) fn lookup_module(&self, s: InternedSymbol) -> RefCell<Rc<Module>> {
        self.modules.get(&s).unwrap().clone()
    }

    fn schedule(&mut self, module: RefCell<Rc<Module>>, code: Vec<u8>) -> Proc {
        self.last_pid = Pid(self.last_pid.0 + 1);
        Proc::new(self.last_pid, module, code)
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
