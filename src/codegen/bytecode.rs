use super::instruction::Instruction;

#[derive(Debug, Clone)]
pub struct BytecodeFunction {
    pub label: String,

    pub captures: Vec<String>,
    pub args: Vec<String>,
    pub locals: Vec<String>,

    pub body: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub struct BytecodeProgram {
    pub globals: Vec<String>,
    pub functions: Vec<BytecodeFunction>,
    pub entry_function_label: String,
}

impl BytecodeProgram {
    pub fn new(entry_function_label: impl Into<String>) -> Self {
        Self {
            globals: Vec::new(),
            functions: Vec::new(),
            entry_function_label: entry_function_label.into(),
        }
    }

    pub fn add_global(&mut self, label: impl Into<String>) {
        self.globals.push(label.into());
    }

    pub fn add_function(&mut self, function: BytecodeFunction) {
        self.functions.push(function);
    }
}

impl BytecodeFunction {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            captures: Vec::new(),
            args: Vec::new(),
            locals: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn emit(&mut self, instruction: Instruction) {
        self.body.push(instruction);
    }
}
