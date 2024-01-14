use num_derive::FromPrimitive;

#[derive(FromPrimitive)]
#[repr(u8)]
pub(crate) enum Op {
    Nop = 0,
    ImmediateSymbol = 1,
    ImmediateInteger = 2,
    ImmediateFloat = 3,
    ImmediateString = 4,
    ConsList = 5,
    ConsVec = 6,
}
