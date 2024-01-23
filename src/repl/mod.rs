use std::error::Error;

use rustyline::{error::ReadlineError, DefaultEditor};

use crate::compiler::compile_one;
use crate::parser::{self, Node};
use crate::vm::Vm;

const HISTORY_FILE: &str = ".alia_history";

pub(crate) fn main(_args: Vec<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("alia repl");

    let mut vm = Vm::new();
    let mut active_module = vm.anonymous_module("*scratch*");

    let mut rl = DefaultEditor::new()?;
    _ = rl.load_history(HISTORY_FILE);

    let mut acc = String::new();
    loop {
        let promptchar = if acc.is_empty() { '>' } else { '*' };
        match rl.readline(&format!("({}){promptchar} ", &active_module.borrow().name)) {
            Ok(line) => {
                let full = acc.clone() + &line;
                match full.parse::<Node>() {
                    Ok(node) => {
                        _ = rl.add_history_entry(&full);
                        acc.clear();
                        let code = compile_one(&node)?;
                        hexdump::hexdump(&code);
                        let mut stack = vm.run_to_completion(active_module.clone(), code);
                        while let Some(val) = stack.pop() {
                            eprintln!("{}", val.format(&vm.interns));
                        }
                    }
                    Err(parser::Error {
                        kind: parser::ErrorKind::Unfinished,
                        ..
                    }) => {
                        acc.push_str(&line);
                        acc.push_str("\n");
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
