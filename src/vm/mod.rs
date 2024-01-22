mod builtins;
mod interns;
mod module;
mod ops;
mod proc;
mod val;

use std::collections::HashMap;
use std::mem;

pub(crate) use self::ops::Op;

use self::interns::{InternedSymbol, Interns};
use self::module::Module;
use self::proc::{Pid, Proc, Step};
use self::val::Val;

pub(crate) struct Vm {
    modules: HashMap<InternedSymbol, Module>,
    pub(super) interns: Interns,
    procs: Vec<Proc>,
    last_pid: Pid,
    completions: HashMap<Pid, Vec<Val>>,
}

impl Vm {
    pub(crate) fn new() -> Self {
        let mut modules = HashMap::new();
        let mut interns = Interns::new();

        modules.insert(interns.intern("ns"), Module::builtins(&mut interns));

        Vm {
            modules,
            interns,
            procs: vec![],
            last_pid: Pid(0),
            completions: HashMap::new(),
        }
    }

    pub(crate) fn run_to_completion(&mut self, code: Vec<u8>) -> Vec<Val> {
        let pid = self.schedule(code);
        while !self.completions.contains_key(&pid) {
            self.step_all();
        }
        self.completions.remove(&pid).unwrap()
    }

    fn schedule(&mut self, code: Vec<u8>) -> Pid {
        self.last_pid = Pid(self.last_pid.0 + 1);
        self.procs.push(Proc::new(self.last_pid, code));
        self.last_pid
    }

    fn step_all(&mut self) {
        self.procs.retain_mut(|p| {
            // we'll probably have to give more soon,
            match p.step(&mut self.interns) {
                Step::Running => true,
                Step::Finished => {
                    self.completions.insert(p.pid, mem::take(&mut p.stack));
                    false
                }
            }
        })
    }
}
