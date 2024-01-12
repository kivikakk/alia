use std::error::Error;

use rustyline::{error::ReadlineError, DefaultEditor};

const HISTORY_FILE: &str = ".alia_history";

pub(crate) fn main(_args: Vec<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("alia repl");

    let mut rl = DefaultEditor::new()?;
    _ = rl.load_history(HISTORY_FILE);

    let active_ns = "*scratch*";
    loop {
        match rl.readline(&format!("({active_ns})> ")) {
            Ok(line) => {
                _ = rl.add_history_entry(&line);
                println!("line: {line}");
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
