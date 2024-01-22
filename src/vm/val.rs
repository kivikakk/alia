use std::rc::Rc;
use std::str;
use std::{cell::RefCell, fmt::Write};

use super::{module::Module, InternedSymbol, Interns};

#[derive(Clone)]
pub(crate) enum Val {
    Symbol(InternedSymbol),
    Integer(i64),
    Float(f64),
    String(String),
    List(Vec<Val>),
    Vec(Vec<Val>),
    Builtin(BuiltinVal),
    Module(RefCell<Rc<Module>>),
}

#[derive(Clone)]
pub(crate) struct BuiltinVal {
    pub(crate) name: String,
    pub(crate) code: Builtin,
}

pub(crate) type Builtin = fn(&mut Interns, Val) -> Val;

impl Val {
    pub(crate) fn format(&self, interns: &Interns) -> String {
        match self {
            Val::Symbol(i) => str::from_utf8(interns.resolve(*i))
                .expect("all symbols should be utf-8")
                .to_string(),
            Val::Integer(i) => format!("{}", i),
            Val::Float(f) => format!("{}", f), // XXX
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
                    write!(s, "{}", n.format(interns)).unwrap();
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
                    write!(s, "{}", n.format(interns)).unwrap();
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
        Val::Symbol(value)
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

impl From<(&str, fn(&mut Interns, Val) -> Val)> for Val {
    fn from(value: (&str, fn(&mut Interns, Val) -> Val)) -> Self {
        Val::Builtin(BuiltinVal {
            name: value.0.into(),
            code: value.1,
        })
    }
}