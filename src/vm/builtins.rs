use super::{interns::Interns, Val};

pub(super) fn print(interns: &mut Interns, arg: Val) -> Val {
    println!("{}", arg.format(interns));
    interns.intern("nil".as_bytes()).into()
}
