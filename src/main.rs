#[cfg(feature = "lsp")]
mod lsp;
mod parser;
#[cfg(feature = "repl")]
mod repl;
mod vm;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut args_it = std::env::args().skip(1).into_iter();
    while let Some(arg) = args_it.next() {
        #[allow(unreachable_code)]
        if arg == "lsp" {
            #[cfg(feature = "lsp")]
            return lsp::main(args_it.collect());
            return Err("lsp feature not built".into());
        } else if arg == "repl" {
            #[cfg(feature = "repl")]
            return repl::main(args_it.collect());
            return Err("repl feature not built".into());
        } else {
            return Err(format!("unknown option {arg:?}").into());
        }
    }

    Err("no usage yet, try alia repl".into())
}
