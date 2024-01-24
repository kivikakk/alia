use num_traits::FromPrimitive;
use std::io::{self, Write};
use std::str;

use crate::vm::Op;

pub(crate) fn disasm(code: &[u8]) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    write!(stdout, "{}", Disassembler::new(code).disasm())?;
    Ok(())
}

struct Disassembler<'c> {
    code: &'c [u8],
    ip: usize,
}

impl<'c> Disassembler<'c> {
    fn new(code: &'c [u8]) -> Disassembler<'c> {
        Disassembler { code, ip: 0 }
    }

    fn disasm(&mut self) -> String {
        let mut out = vec![];

        while self.ip < self.code.len() {
            let op = Op::from_u8(self.code[self.ip])
                .ok_or_else(|| format!("invalid opcode {}", self.code[self.ip]))
                .unwrap();
            self.ip += 1;

            match op {
                Op::Nop => writeln!(out, "{op}").unwrap(),
                Op::ImmediateSymbolBare => {
                    let s = self.bytes();
                    writeln!(out, "{op} {s:?}").unwrap();
                }
                Op::ImmediateSymbolWithModule => {
                    let m = self.bytes();
                    let s = self.bytes();
                    writeln!(out, "{op} {m:?} {s:?}").unwrap();
                }
                Op::ImmediateBooleanTrue => writeln!(out, "{op}").unwrap(),
                Op::ImmediateBooleanFalse => writeln!(out, "{op}").unwrap(),
                Op::ImmediateInteger => {
                    let i = self.n();
                    writeln!(out, "{op} {i:?}").unwrap();
                }
                Op::ImmediateFloat => {
                    let f = self.n();
                    writeln!(out, "{op} {f:?}").unwrap();
                }
                Op::ImmediateString => {
                    let s = self.bytes();
                    writeln!(out, "{op} {s:?}").unwrap();
                }
                Op::ConsList => {
                    let n = self.n();
                    writeln!(out, "{op} {n:?}").unwrap();
                }
                Op::ConsVec => {
                    let n = self.n();
                    writeln!(out, "{op} {n:?}").unwrap();
                }
                Op::Drop => writeln!(out, "{op}").unwrap(),
                Op::Eval => writeln!(out, "{op}").unwrap(),
                Op::Call => {
                    let n = self.n();
                    writeln!(out, "{op} {n:?}").unwrap();
                }
                Op::JumpRelative => {
                    let n = self.n();
                    writeln!(out, "{op} {n:?}").unwrap();
                }
            }
        }

        String::from_utf8(out).unwrap()
    }

    fn n(&mut self) -> usize {
        let n = usize::from_le_bytes(self.code[self.ip..self.ip + 8].try_into().unwrap());
        self.ip += 8;
        n
    }

    fn bytes(&mut self) -> &'c str {
        let n = self.n();
        let s =
            str::from_utf8(&self.code[self.ip..self.ip + n]).expect("source should be valid utf-8");
        self.ip += n;
        s
    }
}
