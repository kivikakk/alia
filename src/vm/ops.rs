use num_derive::FromPrimitive;

#[derive(FromPrimitive)]
#[repr(u8)]
pub(crate) enum Op {
    Nop = 0,
    ImmediateSymbolBare = 1,
    ImmediateSymbolWithModule = 2,
    ImmediateInteger = 3,
    ImmediateFloat = 4,
    ImmediateString = 5,
    ConsList = 6,
    ConsVec = 7,
    Eval = 8,
    JumpRelative = 9,
}
