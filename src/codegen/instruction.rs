#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // Constants
    LoadNull,
    LoadInt(i64),
    LoadReal(f64),
    LoadBool(bool),
    LoadAtom(String),
    LoadFunc(String),

    // Lists
    MakeList(usize),

    // Variables
    LoadLocal(String),
    LoadArg(String),
    LoadCapture(String),
    LoadGlobal(String),

    SetLocal(String),
    SetArg(String),
    SetCapture(String),
    SetGlobal(String),

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Comparisons
    Eq,
    Neq,
    Less,
    Leq,
    Greater,
    Geq,

    // Control flow
    Label(String),
    Jump(String),
    CondJump(String),

    // Calls
    Call(String),

    CallStack { argc: usize },

    // Stack
    Pop,

    // Return
    Ret,
}
