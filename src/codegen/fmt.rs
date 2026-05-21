use std::fmt;

use super::instruction::Instruction;

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::LoadNull => write!(f, "loadnull"),
            Instruction::LoadInt(value) => write!(f, "loadint {}", value),
            Instruction::LoadReal(value) => write!(f, "loadreal {}", value),
            Instruction::LoadBool(value) => write!(f, "loadbool {}", value),
            Instruction::LoadAtom(name) => write!(f, "loadatom {}", name),
            Instruction::LoadFunc(label) => write!(f, "loadfunc {}", label),

            Instruction::MakeList(count) => write!(f, "makelist {}", count),

            Instruction::LoadLocal(label) => write!(f, "loadlocal {}", label),
            Instruction::LoadArg(label) => write!(f, "loadarg {}", label),
            Instruction::LoadCapture(label) => write!(f, "loadcapture {}", label),
            Instruction::LoadGlobal(label) => write!(f, "loadglobal {}", label),

            Instruction::SetLocal(label) => write!(f, "setlocal {}", label),
            Instruction::SetArg(label) => write!(f, "setarg {}", label),
            Instruction::SetCapture(label) => write!(f, "setcapture {}", label),
            Instruction::SetGlobal(label) => write!(f, "setglobal {}", label),

            Instruction::Add => write!(f, "add"),
            Instruction::Sub => write!(f, "sub"),
            Instruction::Mul => write!(f, "mul"),
            Instruction::Div => write!(f, "div"),
            Instruction::Mod => write!(f, "mod"),

            Instruction::Eq => write!(f, "eq"),
            Instruction::Neq => write!(f, "neq"),
            Instruction::Less => write!(f, "less"),
            Instruction::Leq => write!(f, "leq"),
            Instruction::Greater => write!(f, "greater"),
            Instruction::Geq => write!(f, "geq"),

            Instruction::Label(label) => write!(f, "{}:", label),
            Instruction::Jump(label) => write!(f, "jump {}", label),
            Instruction::CondJump(label) => write!(f, "condjump {}", label),

            Instruction::Call { label, argc } => write!(f, "call {} {}", label, argc),
            Instruction::CallStack { argc } => write!(f, "callstack {}", argc),

            Instruction::Pop => write!(f, "pop"),

            Instruction::Ret => write!(f, "ret"),
        }
    }
}

use super::bytecode::{BytecodeFunction, BytecodeProgram};

impl fmt::Display for BytecodeProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for global in &self.globals {
            writeln!(f, "global {}", global)?;
        }

        if !self.globals.is_empty() {
            writeln!(f)?;
        }

        writeln!(f, "entry {}", self.entry_function_label)?;
        writeln!(f)?;

        for function in &self.functions {
            writeln!(f, "{}", function)?;
        }

        Ok(())
    }
}

impl fmt::Display for BytecodeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "func {} {{", self.label)?;

        for capture in &self.captures {
            writeln!(f, "    capture {}", capture)?;
        }

        for arg in &self.args {
            writeln!(f, "    arg {}", arg)?;
        }

        for local in &self.locals {
            writeln!(f, "    local {}", local)?;
        }

        if !self.captures.is_empty() || !self.args.is_empty() || !self.locals.is_empty() {
            writeln!(f)?;
        }

        for instruction in &self.body {
            writeln!(f, "    {}", instruction)?;
        }

        writeln!(f, "}}")?;
        writeln!(f)
    }
}