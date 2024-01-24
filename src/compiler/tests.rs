#![cfg(test)]

use std::collections::HashMap;

use crate::parser::Document;

struct AsmState {
    out: Vec<u8>,
    markers: HashMap<String, usize>,
}

impl AsmState {
    fn new() -> Self {
        AsmState {
            out: vec![],
            markers: HashMap::new(),
        }
    }

    fn marker(&mut self, label: &str) {
        match self.markers.insert(label.to_string(), self.out.len()) {
            None => {}
            _ => panic!("duplicate marker {label}"),
        }
    }

    fn since(&self, label: &str) -> u64 {
        (self.out.len() - self.markers.get(label).expect("undefined label")) as u64
    }
}

// matches Compiler
macro_rules! asm {
    ($( $rest:tt )+ ) => {{
        let mut state = AsmState::new();
        asm_into!(=> state, { $( $rest  )+ });
        state.out
    }};
}

macro_rules! asm_into {
    (=> $state:ident, { $kind:ident $value:tt; $( $rest:tt )*  }) => {
        asm_into!(=> $state, $kind $value);
        asm_into!(=> $state, { $( $rest )* });
    };

    (=> $state:ident, { $label:ident : $( $rest:tt )* }) => {
        $state.marker(stringify!($label));
        asm_into!(=> $state, { $( $rest )* });
    };

    (=> $state:ident, { rip &$label:ident; $( $rest:tt )* }) => {
        asm_into!(=> $state, { rip &$label (-0); $( $rest )* });
    };
    (=> $state:ident, { rip &$label:ident (-op); $( $rest:tt )* }) => {
        asm_into!(=> $state, { rip &$label (-1); $( $rest )* });
    };
    (=> $state:ident, { rip &$label:ident (-$n:literal); $( $rest:tt )* }) => {
        $state.out.extend_from_slice(&($state.since(stringify!($label)) - $n).to_le_bytes());
        asm_into!(=> $state, { $( $rest )* });
    };

    (=> $state:ident, {  }) => {
        // THAT'LL DO, PIG
    };

    (=> $state:ident, op $op:ident) => {
        $state.out.push($crate::vm::Op::$op as u8);
    };
    (=> $state:ident, n $value:literal) => {
        $state.out.extend_from_slice(&($value as u64).to_le_bytes());
    };

    (=> $state:ident, str $value:literal) => {
        $state.out.extend_from_slice($value.as_bytes());
    };
}

fn assert_compiles<C: AsRef<[u8]>>(code: &str, expected: C) {
    let doc = code.parse::<Document>().unwrap();
    assert_eq!(Ok(expected.as_ref()), doc.compile().as_deref());
}

#[test]
fn boolean_shortcut() {
    assert_compiles("true", asm! { op ImmediateBooleanTrue; op Drop; });
    assert_compiles("false", asm! { op ImmediateBooleanFalse; op Drop; });
}

#[test]
#[ignore = "jump compile NYI"]
fn loop_compiles_just_fine() {
    assert_compiles(
        "(loop (awawa))",
        asm! {
        begin:
            op  ImmediateSymbolBare;
            n   5;
            str "awawa";

            op  ConsList;
            n   1;

            op  Eval;

            op  JumpRelative;
            rip &begin (-op);
        },
    );
}
