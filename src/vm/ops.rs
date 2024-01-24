use num_derive::FromPrimitive;

#[derive(FromPrimitive)]
#[repr(u8)]
pub(crate) enum Op {
    Nop = 0,
    //
    ImmediateSymbolBare = 1,
    ImmediateSymbolWithModule = 2,
    ImmediateBooleanTrue = 3,
    ImmediateBooleanFalse = 4,
    ImmediateInteger = 5,
    ImmediateFloat = 6,
    ImmediateString = 7,
    ConsList = 8,
    ConsVec = 9,
    //
    Drop = 10,
    Eval = 11,
    Call = 12,
    //
    JumpRelative = 20,
}
