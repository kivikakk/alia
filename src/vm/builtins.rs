use super::{proc::Proc, Val, Vm};

fn eval_args(vm: &mut Vm, proc: &mut Proc, args: Vec<&Val>) -> Vec<Val> {
    args.into_iter().map(|f| proc.eval(vm, f)).collect()
}

pub(super) fn print(vm: &mut Vm, proc: &mut Proc, args: Vec<&Val>) -> Val {
    for arg in eval_args(vm, proc, args) {
        println!("{}", arg.format(vm));
    }
    Val::List(vec![])
}

pub(super) fn quote(_vm: &mut Vm, _proc: &mut Proc, args: Vec<&Val>) -> Val {
    let mut args = args.into_iter();
    let head = args.next().expect("quote shouldn't be empty");
    assert!(args.next().is_none(), "quote shouldn't have multiple items");
    head.clone()
}
