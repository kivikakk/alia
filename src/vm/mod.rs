mod interns;
mod mark;
mod ops;
mod proc;
mod val;

use std::collections::HashMap;
use std::mem;

pub(crate) use self::ops::Op;

use self::interns::{InternedSymbol, Interns};
use self::mark::Mark;
use self::proc::{Pid, Proc};
use self::val::Val;

pub(crate) struct Vm {
    marks: HashMap<InternedSymbol, Mark>,
    interns: Interns,
    procs: Vec<Proc>,
    last_pid: Pid,
    completions: HashMap<Pid, Vec<Val>>,
}

impl Vm {
    pub(crate) fn new() -> Self {
        let mut marks = HashMap::new();
        let mut interns = Interns::new();

        marks.insert(interns.intern("builtins"), Mark::builtins());

        Vm {
            marks,
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
            if p.step(&mut self.interns) {
                self.completions.insert(p.pid, mem::take(&mut p.stack));
                false
            } else {
                true
            }
        })
    }
}
