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

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Nop => write!(f, "Nop"),
            Op::ImmediateSymbolBare => write!(f, "ImmediateSymbolBare"),
            Op::ImmediateSymbolWithModule => write!(f, "ImmediateSymbolWithModule"),
            Op::ImmediateBooleanTrue => write!(f, "ImmediateBooleanTrue"),
            Op::ImmediateBooleanFalse => write!(f, "ImmediateBooleanFalse"),
            Op::ImmediateInteger => write!(f, "ImmediateInteger"),
            Op::ImmediateFloat => write!(f, "ImmediateFloat"),
            Op::ImmediateString => write!(f, "ImmediateString"),
            Op::ConsList => write!(f, "ConsList"),
            Op::ConsVec => write!(f, "ConsVec"),
            Op::Drop => write!(f, "Drop"),
            Op::Eval => write!(f, "Eval"),
            Op::Call => write!(f, "Call"),
            Op::JumpRelative => write!(f, "JumpRelative"),
        }
    }
}
