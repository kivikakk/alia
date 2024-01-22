use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{
    val::{Builtin, BuiltinVal},
    InternedSymbol, Val, Vm,
};

#[derive(Clone)]
pub(crate) struct Module {
    // consts // fns // macros
    // ^--- these all occupy the same namespace!
    pub(super) name: String,
    submodules: HashMap<InternedSymbol, RefCell<Rc<Module>>>,
    binds: HashMap<InternedSymbol, Val>,
}

impl Module {
    fn new(name: String) -> Self {
        Module {
            name,
            submodules: HashMap::new(),
            binds: HashMap::new(),
        }
    }

    pub(super) fn builtins(vm: &mut Vm) -> Module {
        let mut m = Module::new("builtins".into());
        m.add_bind(
            vm.interns.intern("print"),
            Val::Builtin(BuiltinVal {
                name: "print".into(),
                code: super::builtins::print,
            }),
        );
        m.add_bind(
            vm.interns.intern("pront"),
            ("pront", super::builtins::print as Builtin).into(),
        );
        m
    }

    pub(super) fn add_bind(&mut self, sym: InternedSymbol, target: Val) {
        match self.binds.insert(sym, target) {
            None => {}
            Some(_) => panic!("duplicate bind"),
        }
    }
}
