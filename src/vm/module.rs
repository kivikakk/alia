use std::collections::HashMap;

use super::{
    interns::Interns,
    val::{Builtin, BuiltinVal},
    InternedSymbol, Val,
};

pub(super) struct Module {
    // consts // fns // macros
    // ^--- these all occupy the same namespace!
    submodules: HashMap<InternedSymbol, Module>,
    binds: HashMap<InternedSymbol, Val>,
}

impl Module {
    fn new() -> Self {
        Module {
            submodules: HashMap::new(),
            binds: HashMap::new(),
        }
    }

    pub(super) fn builtins(is: &mut Interns) -> Module {
        let mut m = Module::new();
        m.add_bind(
            is.intern("print"),
            Val::Builtin(BuiltinVal {
                name: "print".into(),
                code: super::builtins::print,
            }),
        );
        m.add_bind(
            is.intern("pront"),
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
