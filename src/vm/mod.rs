mod builtins;
mod interns;
mod module;
mod ops;
mod proc;
mod val;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str;

pub(crate) use self::interns::InternedSymbol;
pub(crate) use self::module::Module;
pub(crate) use self::ops::Op;
pub(crate) use self::val::{BuiltinVal, Val};

use self::interns::Interns;
use self::proc::{Pid, Proc, Step};

pub(crate) struct Vm {
    pub(super) modules: HashMap<InternedSymbol, Rc<RefCell<Module>>>,
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
            Rc::new(RefCell::new(builtins)),
        );
        vm
    }

    pub(crate) fn intern(&mut self, name: &str) -> InternedSymbol {
        self.interns.intern(name)
    }

    pub(crate) fn resolve(&self, s: InternedSymbol) -> &str {
        str::from_utf8(self.interns.resolve(s)).expect("all interned symbols should be utf-8")
    }

    pub(crate) fn anonymous_module(&mut self, name: &str) -> Rc<RefCell<Module>> {
        let mut module = Module::new(name.to_string());
        module.refer(
            self.modules
                .get(&self.interns.intern("builtins"))
                .unwrap()
                .clone(),
        );
        Rc::new(RefCell::new(module))
    }

    pub(crate) fn run_to_completion(&mut self, module: Rc<RefCell<Module>>, code: Vec<u8>) -> Val {
        let proc = self.schedule(module, code);
        self.step_to_end(proc)
    }

    pub(crate) fn lookup_module(&self, s: InternedSymbol) -> Option<Rc<RefCell<Module>>> {
        self.modules.get(&s).cloned()
    }

    fn schedule(&mut self, module: Rc<RefCell<Module>>, code: Vec<u8>) -> Proc {
        self.last_pid = Pid(self.last_pid.0 + 1);
        Proc::new(self.last_pid, module, code)
    }

    fn step_to_end(&mut self, mut proc: Proc) -> Val {
        loop {
            match proc.step(self) {
                Step::Running => {}
                Step::Finished => {
                    return proc.last.expect("proc should return (drop) a value");
                }
            }
        }
    }
}
