use std::fmt::Write;
use std::str;

use super::{InternedSymbol, Vm};

pub(crate) enum Val {
    Symbol(InternedSymbol),
    Integer(i64),
    Float(f64),
    String(String),
    List(Vec<Val>),
    Vec(Vec<Val>),
}

impl Val {
    pub(crate) fn format(&self, vm: &Vm) -> String {
        match self {
            Val::Symbol(i) => str::from_utf8(vm.interns.resolve(*i))
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
        }
    }
}
