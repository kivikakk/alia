use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{
    val::{Builtin, BuiltinVal},
    InternedSymbol, Val, Vm,
};

#[derive(Clone)]
pub(crate) struct Module {
    // consts // fns // macros
    // ^--- these all occupy the same namespace!
    pub(crate) name: String,
    pub(crate) submodules: HashMap<InternedSymbol, Rc<RefCell<Module>>>,
    pub(crate) refers: Vec<Rc<RefCell<Module>>>,
    pub(crate) binds: HashMap<InternedSymbol, Val>, // XXX
}

impl Module {
    pub(super) fn new(name: String) -> Self {
        Module {
            name,
            submodules: HashMap::new(),
            refers: vec![],
            binds: HashMap::new(),
        }
    }

    pub(super) fn builtins(vm: &mut Vm) -> Module {
        let mut m = Module::new("builtins".into());
        for k in &["true", "false"] {
            let sym = vm.interns.intern(k);
            m.add_bind(sym, Val::Symbol(None, sym));
        }
        m.add_bind_builtin(vm, "print", super::builtins::print);
        m.add_bind_builtin(vm, "quote", super::builtins::quote);
        m
    }

    pub(super) fn refer(&mut self, module: Rc<RefCell<Module>>) {
        self.refers.push(module);
    }

    pub(crate) fn lookup(&self, vm: &Vm, s: InternedSymbol) -> Option<Val> {
        // Order
        // * closure/let binds (TODO)
        // * binds
        // * top-level modules
        // * refers
        //
        // Note that submodules aren't lookup-able
        if let Some(v) = self.binds.get(&s) {
            return Some(v.clone());
        }
        if let Some(m) = vm.modules.get(&s) {
            return Some(Val::Module(m.clone()));
        }
        for rm in &self.refers {
            // XXX "restricted" refer lookup, not a full recurse
            // Not sure if we want this or otherwise.
            if let Some(v) = rm.borrow().binds.get(&s) {
                return Some(v.clone());
            }
        }
        None
    }

    pub(super) fn add_bind(&mut self, name: InternedSymbol, target: Val) {
        match self.binds.insert(name, target) {
            None => {}
            Some(_) => panic!("duplicate bind"),
        }
    }

    pub(crate) fn set(&mut self, vm: &mut Vm, name: &str, target: Val) {
        let s = vm.interns.intern(name);
        self.sets(s, target);
    }

    pub(crate) fn sets(&mut self, s: InternedSymbol, target: Val) {
        _ = self.binds.insert(s, target);
    }

    pub(super) fn add_bind_builtin(&mut self, vm: &mut Vm, name: &str, target: Builtin) {
        self.add_bind(
            vm.interns.intern(name),
            Val::Builtin(BuiltinVal {
                name: format!("builtins/{name}"),
                code: target,
            }),
        );
    }
}
