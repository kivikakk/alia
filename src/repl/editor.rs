use std::{cell::RefCell, rc::Rc, str};

use rustyline::{
    completion::Completer, highlight::Highlighter, hint::Hinter, history::DefaultHistory,
    line_buffer::LineBuffer, validate::Validator, Changeset, Context, Helper, Result,
};

use crate::vm::{InternedSymbol, Module, Vm};

pub(super) type Editor = rustyline::Editor<EditorHelper, DefaultHistory>;

pub(super) struct EditorHelper {
    vm: Rc<RefCell<Vm>>,
    active_module: Rc<RefCell<Module>>,
}

impl EditorHelper {
    pub(super) fn new(vm: Rc<RefCell<Vm>>, active_module: Rc<RefCell<Module>>) -> EditorHelper {
        EditorHelper { vm, active_module }
    }
}

impl Helper for EditorHelper {}

fn symbol_char(c: u8) -> bool {
    // matches lexer.re, except we add '/'
    matches!(c, b'a' ..= b'z' | b'A' ..= b'Z' | b'*' | b'_' | b'<' | b'>' | b'!' | b'=' | b'+' | b'-' | b'/')
}

fn complete_in_module(vm: &Vm, module: &Module, entry: &str) -> Vec<String> {
    // matches Module::lookup
    let mut results = vec![];
    complete_from(vm, module.binds.keys().cloned(), entry, &mut results);
    complete_from(vm, vm.modules.keys().cloned(), entry, &mut results);
    for rm in &module.refers {
        complete_from(vm, rm.borrow().binds.keys().cloned(), entry, &mut results);
    }
    results
}

fn complete_from(
    vm: &Vm,
    source: impl Iterator<Item = InternedSymbol>,
    entry: &str,
    results: &mut Vec<String>,
) {
    for s in source {
        let s = vm.resolve(s);
        if let Some(suffix) = s.strip_prefix(entry) {
            results.push(suffix.to_string());
        }
    }
}

impl Completer for EditorHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>)> {
        // Support minimal symbol tab completion:
        // 1. at end of line only        <--
        // 2. anywhere
        // 3. actually using our lexer
        let line = line.as_bytes();
        if pos != line.len() {
            return Ok((0, Vec::with_capacity(0)));
        }
        let mut start = pos;
        while start != 0 {
            start -= 1;
            if !symbol_char(line[start]) {
                start += 1;
                break;
            }
        }

        let entry = str::from_utf8(&line[start..]).expect("source should be valid utf-8");

        // matches Proc::eval
        let mut vm = self.vm.borrow_mut();
        match entry.split_once('/') {
            Some((m, s)) => {
                let m = vm.intern(m);
                let module = match vm.lookup_module(m) {
                    Some(v) => v,
                    None => return Ok((0, Vec::with_capacity(0))),
                };
                let module = module.borrow();
                Ok((pos, complete_in_module(&vm, &module, s)))
            }
            None => {
                let am = self.active_module.borrow();
                Ok((pos, complete_in_module(&vm, &am, entry)))
            }
        }
    }

    fn update(&self, line: &mut LineBuffer, start: usize, elected: &str, cl: &mut Changeset) {
        let end = line.pos();
        line.replace(start..end, elected, cl);
    }
}

impl Hinter for EditorHelper {
    type Hint = String;
}

impl Highlighter for EditorHelper {}

impl Validator for EditorHelper {}
