use std::error::Error;

use rustyline::{error::ReadlineError, DefaultEditor};

use crate::parser::{Node, ParseError, ParseErrorKind};

const HISTORY_FILE: &str = ".alia_history";

pub(crate) fn main(_args: Vec<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("alia repl");

    let mut rl = DefaultEditor::new()?;
    _ = rl.load_history(HISTORY_FILE);

    let active_ns = "*scratch*";
    let mut acc = String::new();
    loop {
        let promptchar = if acc.is_empty() { '>' } else { '*' };
        match rl.readline(&format!("({active_ns}){promptchar} ")) {
            Ok(line) => {
                let full = acc.clone() + &line;
                match full.parse::<Node>() {
                    Ok(node) => {
                        _ = rl.add_history_entry(&full);
                        println!("{node}");
                        acc.clear();
                    }
                    Err(ParseError {
                        kind: ParseErrorKind::Unfinished,
                        ..
                    }) => {
                        acc.push_str(&line);
                        acc.push_str("\n");
                    }
                    Err(ParseError {
                        kind: ParseErrorKind::Empty,
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
