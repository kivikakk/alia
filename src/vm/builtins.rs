use crate::parser::Document;

use super::{proc::Proc, Module, Val, Vm};

fn eval_args(vm: &mut Vm, proc: &mut Proc, args: &[Val]) -> Vec<Val> {
    args.into_iter().map(|f| proc.eval(vm, f)).collect()
}

pub(super) fn add_all(vm: &mut Vm, m: &mut Module) {
    m.add_bind_builtin(vm, "print", print);
    m.add_bind_builtin(vm, "quote", quote);
    m.add_bind_builtin(vm, "set", set);
    m.add_bind_builtin(vm, "eval", eval);
}

fn print(vm: &mut Vm, proc: &mut Proc, args: &[Val]) -> Val {
    // (print "a") => ()
    //   ; prints 'a' as a side-effect

    for arg in eval_args(vm, proc, args) {
        println!("{}", arg.format(vm));
    }
    Val::List(Vec::with_capacity(0))
}

fn quote(_vm: &mut Vm, _proc: &mut Proc, args: &[Val]) -> Val {
    // (quote x) => x

    let mut args = args.into_iter().cloned();
    let head = args.next().expect("quote shouldn't be empty");
    assert!(args.next().is_none(), "quote shouldn't have multiple items");
    head.clone()
}

fn set(vm: &mut Vm, proc: &mut Proc, args: &[Val]) -> Val {
    // (set x 1) => 1
    //   ; sets the local bind x to 1 as a side-effect

    if args.len() != 2 {
        panic!("set should take two arguments");
    }
    let s = match args[0] {
        Val::Symbol(None, s) => s,
        _ => panic!("trying to set {}", args[0].format(vm)),
    };
    let v = proc.eval(vm, &args[1]);
    _ = proc.module.borrow_mut().binds.insert(s, v.clone());
    v
}

fn eval(vm: &mut Vm, proc: &mut Proc, args: &[Val]) -> Val {
    // (eval "print") => builtins/print

    if args.len() != 1 {
        panic!("eval should take one argument");
    }

    let s = match &args[0] {
        Val::String(s) => s,
        _ => panic!("trying to eval {}", args[0].format(vm)),
    };
    let doc = match s.parse::<Document>() {
        Ok(node) => node,
        _ => panic!("failed to parse {s:?}"),
    };
    let code = doc.compile().unwrap();
    vm.run_to_completion(proc.module.clone(), code)
}
