mod lsp;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut args_it = std::env::args().skip(1).into_iter();
    while let Some(arg) = args_it.next() {
        if arg == "--lsp" {
            return lsp::main(args_it.collect());
        } else {
            return Err(format!("unknown option {arg:?}").into());
        }
    }

    eprintln!("alia");
    Ok(())
}
