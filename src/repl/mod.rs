mod editor;

use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use rustyline::error::ReadlineError;

use crate::disasm::disasm;
use crate::parser::{self, Document};
use crate::vm::{Val, Vm};

use self::editor::{Editor, EditorHelper};

const HISTORY_FILE: &str = ".alia_history";

pub(crate) fn main(_args: Vec<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("alia repl");

    let mut vm = Vm::new();
    let active_module = vm.anonymous_module("*scratch*");
    let strue = vm.intern("true");
    let _sfalse = vm.intern("false");
    let sareb = vm.intern("alia-repl-echo-bytecode");
    active_module
        .borrow_mut()
        .sets(sareb, Val::Symbol(None, strue));

    let vm = Rc::new(RefCell::new(vm));

    let mut rl = Editor::new()?;
    rl.set_helper(Some(EditorHelper::new(vm.clone(), active_module.clone())));
    // XXX currently there's no sync of EditorHelper and our active_module here.
    _ = rl.load_history(HISTORY_FILE);

    let mut acc = String::new();
    loop {
        let promptchar = if acc.is_empty() { '>' } else { '*' };
        match rl.readline(&format!("({}){promptchar} ", &active_module.borrow().name)) {
            Ok(line) => {
                let full = acc.clone() + &line;
                match full.parse::<Document>() {
                    Ok(doc) => {
                        _ = rl.add_history_entry(&full);
                        acc.clear();
                        let code = doc.compile()?;
                        let mut vm = vm.borrow_mut();
                        match active_module.borrow().lookup(&vm, sareb) {
                            Some(Val::Symbol(None, s)) if s == strue => {
                                disasm(&code)?;
                            }
                            _ => {}
                        }
                        let val = vm.run_to_completion(active_module.clone(), code);
                        eprintln!("{}", val.format(&vm));
                    }
                    Err(parser::Error {
                        kind: parser::ErrorKind::Unfinished,
                        ..
                    }) => {
                        acc.push_str(&line);
                        acc.push('\n');
                    }
                    Err(parser::Error {
                        kind: parser::ErrorKind::Empty,
                        ..
                    }) => {}
                    Err(err) => {
                        println!("error: {err}");
                        acc.clear();
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                eprintln!("^C");
                break;
            }
            Err(ReadlineError::Eof) => {
                eprintln!("^D");
                break;
            }
            Err(err) => {
                eprintln!("error: {err:?}");
                break;
            }
        }
    }

    rl.save_history(HISTORY_FILE)?;

    Ok(())
}
