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
    submodules: HashMap<InternedSymbol, RefCell<Rc<Module>>>,
    refers: Vec<RefCell<Rc<Module>>>,
    binds: HashMap<InternedSymbol, Val>,
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
        m.add_bind_builtin(vm, "pront", super::builtins::print);
        m
    }

    pub(super) fn refer(&mut self, module: RefCell<Rc<Module>>) {
        self.refers.push(module);
    }

    pub(super) fn lookup(&self, vm: &Vm, s: InternedSymbol) -> Option<Val> {
        // Order
        // * closure/let binds (TODO)
        // * binds
        // * top-level modules
        // * refers
        //
        // Note that submodules aren't lookup-able bare.
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

    pub(super) fn add_bind(&mut self, sym: InternedSymbol, target: Val) {
        match self.binds.insert(sym, target) {
            None => {}
            Some(_) => panic!("duplicate bind"),
        }
    }

    pub(super) fn add_bind_builtin(&mut self, vm: &mut Vm, name: &str, target: Builtin) {
        self.add_bind(
            vm.interns.intern(name),
            Val::Builtin(BuiltinVal {
                name: name.to_string(),
                code: target,
            }),
        );
    }
}
