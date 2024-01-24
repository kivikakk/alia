use std::rc::Rc;
use std::str;
use std::{cell::RefCell, fmt::Write};

use super::proc::Proc;
use super::Vm;
use super::{module::Module, InternedSymbol};

#[derive(Clone)]
pub(crate) enum Val {
    Symbol(Option<InternedSymbol>, InternedSymbol),
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    List(Vec<Val>),
    Vec(Vec<Val>),
    Builtin(BuiltinVal),
    Module(Rc<RefCell<Module>>),
}

#[derive(Clone)]
pub(crate) struct BuiltinVal {
    pub(crate) name: String,
    pub(crate) code: Builtin,
}

pub(crate) type Builtin = fn(&mut Vm, &mut Proc, &[Val]) -> Val;

impl Val {
    pub(crate) fn format(&self, vm: &Vm) -> String {
        match self {
            &Val::Symbol(None, s) => str::from_utf8(vm.interns.resolve(s))
                .expect("all symbols should be utf-8")
                .to_string(),
            &Val::Boolean(b) => format!("{b}"),
            &Val::Symbol(Some(m), s) => format!(
                "{}/{}",
                str::from_utf8(vm.interns.resolve(m)).expect("all symbol modules should be utf-8"),
                str::from_utf8(vm.interns.resolve(s)).expect("all symbols should be utf-8"),
            ),
            Val::Integer(i) => format!("{}", i),
            Val::Float(f) => format!("{}", f), // XXX doesn't roundtrip
            Val::String(s) => s.to_string(),
            Val::List(ns) => {
                let mut s = "(".to_string();
                let mut first = true;
                for n in ns {
                    if first {
                        first = false;
                    } else {
                        write!(s, " ").unwrap();
                    }
                    write!(s, "{}", n.format(vm)).unwrap();
                }
                write!(s, ")").unwrap();
                s
            }
            Val::Vec(ns) => {
                let mut s = "[".to_string();
                let mut first = true;
                for n in ns {
                    if first {
                        first = false;
                    } else {
                        write!(s, " ").unwrap();
                    }
                    write!(s, "{}", n.format(vm)).unwrap();
                }
                write!(s, "]").unwrap();
                s
            }
            Val::Builtin(BuiltinVal { name, .. }) => {
                format!("<builtin {name}>")
            }
            Val::Module(rmod) => {
                let name = &rmod.borrow().name;
                format!("<module {name}>")
            }
        }
    }
}

impl From<InternedSymbol> for Val {
    fn from(value: InternedSymbol) -> Self {
        Val::Symbol(None, value)
    }
}

impl From<(InternedSymbol, InternedSymbol)> for Val {
    fn from(value: (InternedSymbol, InternedSymbol)) -> Self {
        Val::Symbol(Some(value.0), value.1)
    }
}

impl From<(Option<InternedSymbol>, InternedSymbol)> for Val {
    fn from(value: (Option<InternedSymbol>, InternedSymbol)) -> Self {
        Val::Symbol(value.0, value.1)
    }
}

impl From<i64> for Val {
    fn from(value: i64) -> Self {
        Val::Integer(value)
    }
}

impl From<f64> for Val {
    fn from(value: f64) -> Self {
        Val::Float(value)
    }
}

impl From<String> for Val {
    fn from(value: String) -> Self {
        Val::String(value)
    }
}

// XXX: List/Vec are undistinguished

impl From<(&str, Builtin)> for Val {
    fn from(value: (&str, Builtin)) -> Self {
        Val::Builtin(BuiltinVal {
            name: value.0.into(),
            code: value.1,
        })
    }
}
