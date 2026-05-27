use std::collections::HashMap;
use std::fmt;

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

    // Stdout
    Stdout,

    // Return
    Ret,

    // VM builtins
    Head,
    Tail,
    Cons,
    IsNull,
    Length,
    Or,
    Not,
    IsList,
    Eval,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub label: String,
    pub captures: Vec<String>,
    pub args: Vec<String>,
    pub locals: Vec<String>,
    pub body: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub globals: Vec<String>,
    pub functions: HashMap<String, Function>,
    pub entry: String,
}

pub fn parse(input: &str) -> Result<Program, String> {
    let mut globals = Vec::new();
    let mut functions = HashMap::new();
    let mut entry = String::new();

    let mut lines = input.lines().peekable();

    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed.starts_with("global ") {
            globals.push(trimmed[7..].trim().to_string());
        } else if trimmed.starts_with("entry ") {
            entry = trimmed[6..].trim().to_string();
        } else if trimmed.starts_with("func ") {
            let label = trimmed[5..]
                .trim()
                .strip_suffix("{")
                .map(|s| s.trim().to_string())
                .ok_or_else(|| format!("Invalid func declaration: {}", trimmed))?;
            let mut func = Function {
                label: label.clone(),
                captures: Vec::new(),
                args: Vec::new(),
                locals: Vec::new(),
                body: Vec::new(),
            };

            while let Some(inner) = lines.next() {
                let inner_trimmed = inner.trim();
                if inner_trimmed.is_empty() {
                    continue;
                }
                if inner_trimmed == "}" {
                    break;
                }

                if inner_trimmed.starts_with("capture ") {
                    func.captures.push(inner_trimmed[8..].trim().to_string());
                } else if inner_trimmed.starts_with("arg ") {
                    func.args.push(inner_trimmed[4..].trim().to_string());
                } else if inner_trimmed.starts_with("local ") {
                    func.locals.push(inner_trimmed[6..].trim().to_string());
                } else {
                    func.body.push(parse_instruction(inner_trimmed)?);
                }
            }

            functions.insert(label, func);
        } else {
            return Err(format!("Unexpected top-level line: {}", trimmed));
        }
    }

    if entry.is_empty() {
        return Err("Missing entry function".to_string());
    }

    Ok(Program {
        globals,
        functions,
        entry,
    })
}

fn parse_instruction(line: &str) -> Result<Instruction, String> {
    if line.ends_with(':') {
        return Ok(Instruction::Label(line[..line.len() - 1].to_string()));
    }

    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return Err("Empty instruction".to_string());
    }

    match parts[0] {
        "loadnull" => Ok(Instruction::LoadNull),
        "loadint" => {
            let v = parts.get(1).ok_or("Missing loadint arg")?;
            Ok(Instruction::LoadInt(
                v.parse()
                    .map_err(|e| format!("Invalid int: {} ({})", v, e))?,
            ))
        }
        "loadreal" => {
            let v = parts.get(1).ok_or("Missing loadreal arg")?;
            Ok(Instruction::LoadReal(
                v.parse()
                    .map_err(|e| format!("Invalid real: {} ({})", v, e))?,
            ))
        }
        "loadbool" => {
            let v = parts.get(1).ok_or("Missing loadbool arg")?;
            match *v {
                "true" => Ok(Instruction::LoadBool(true)),
                "false" => Ok(Instruction::LoadBool(false)),
                _ => Err(format!("Invalid bool: {}", v)),
            }
        }
        "loadatom" => {
            let v = parts.get(1).ok_or("Missing loadatom arg")?;
            Ok(Instruction::LoadAtom(v.to_string()))
        }
        "loadfunc" => {
            let v = parts.get(1).ok_or("Missing loadfunc arg")?;
            Ok(Instruction::LoadFunc(v.to_string()))
        }
        "makelist" => {
            let v = parts.get(1).ok_or("Missing makelist arg")?;
            Ok(Instruction::MakeList(
                v.parse()
                    .map_err(|e| format!("Invalid makelist: {} ({})", v, e))?,
            ))
        }
        "loadlocal" => Ok(Instruction::LoadLocal(parts[1..].join(" "))),
        "loadarg" => Ok(Instruction::LoadArg(parts[1..].join(" "))),
        "loadcapture" => Ok(Instruction::LoadCapture(parts[1..].join(" "))),
        "loadglobal" => Ok(Instruction::LoadGlobal(parts[1..].join(" "))),
        "setlocal" => Ok(Instruction::SetLocal(parts[1..].join(" "))),
        "setarg" => Ok(Instruction::SetArg(parts[1..].join(" "))),
        "setcapture" => Ok(Instruction::SetCapture(parts[1..].join(" "))),
        "setglobal" => Ok(Instruction::SetGlobal(parts[1..].join(" "))),
        "add" => Ok(Instruction::Add),
        "sub" => Ok(Instruction::Sub),
        "mul" => Ok(Instruction::Mul),
        "div" => Ok(Instruction::Div),
        "mod" => Ok(Instruction::Mod),
        "eq" => Ok(Instruction::Eq),
        "neq" => Ok(Instruction::Neq),
        "less" => Ok(Instruction::Less),
        "leq" => Ok(Instruction::Leq),
        "greater" => Ok(Instruction::Greater),
        "geq" => Ok(Instruction::Geq),
        "jump" => Ok(Instruction::Jump(parts[1..].join(" "))),
        "condjump" => Ok(Instruction::CondJump(parts[1..].join(" "))),
        "call" => Ok(Instruction::Call(parts[1..].join(" "))),
        "callstack" => {
            let v = parts.get(1).ok_or("Missing callstack arg")?;
            Ok(Instruction::CallStack {
                argc: v
                    .parse()
                    .map_err(|e| format!("Invalid callstack: {} ({})", v, e))?,
            })
        }
        "pop" => Ok(Instruction::Pop),
        "stdout" => Ok(Instruction::Stdout),
        "ret" => Ok(Instruction::Ret),
        "head" => Ok(Instruction::Head),
        "tail" => Ok(Instruction::Tail),
        "cons" => Ok(Instruction::Cons),
        "isnull" => Ok(Instruction::IsNull),
        "length" => Ok(Instruction::Length),
        "or" => Ok(Instruction::Or),
        "not" => Ok(Instruction::Not),
        "islist" => Ok(Instruction::IsList),
        "eval" => Ok(Instruction::Eval),
        _ => Err(format!("Unknown instruction: {}", parts[0])),
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::LoadNull => write!(f, "loadnull"),
            Instruction::LoadInt(v) => write!(f, "loadint {}", v),
            Instruction::LoadReal(v) => write!(f, "loadreal {}", v),
            Instruction::LoadBool(v) => write!(f, "loadbool {}", v),
            Instruction::LoadAtom(v) => write!(f, "loadatom {}", v),
            Instruction::LoadFunc(v) => write!(f, "loadfunc {}", v),
            Instruction::MakeList(v) => write!(f, "makelist {}", v),
            Instruction::LoadLocal(v) => write!(f, "loadlocal {}", v),
            Instruction::LoadArg(v) => write!(f, "loadarg {}", v),
            Instruction::LoadCapture(v) => write!(f, "loadcapture {}", v),
            Instruction::LoadGlobal(v) => write!(f, "loadglobal {}", v),
            Instruction::SetLocal(v) => write!(f, "setlocal {}", v),
            Instruction::SetArg(v) => write!(f, "setarg {}", v),
            Instruction::SetCapture(v) => write!(f, "setcapture {}", v),
            Instruction::SetGlobal(v) => write!(f, "setglobal {}", v),
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
            Instruction::Label(v) => write!(f, "{}:", v),
            Instruction::Jump(v) => write!(f, "jump {}", v),
            Instruction::CondJump(v) => write!(f, "condjump {}", v),
            Instruction::Call(v) => write!(f, "call {}", v),
            Instruction::CallStack { argc } => write!(f, "callstack {}", argc),
            Instruction::Pop => write!(f, "pop"),
            Instruction::Stdout => write!(f, "stdout"),
            Instruction::Ret => write!(f, "ret"),
            Instruction::Head => write!(f, "head"),
            Instruction::Tail => write!(f, "tail"),
            Instruction::Cons => write!(f, "cons"),
            Instruction::IsNull => write!(f, "isnull"),
            Instruction::Length => write!(f, "length"),
            Instruction::Or => write!(f, "or"),
            Instruction::Not => write!(f, "not"),
            Instruction::IsList => write!(f, "islist"),
            Instruction::Eval => write!(f, "eval"),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "func {} {{", self.label)?;
        for cap in &self.captures {
            writeln!(f, "    capture {}", cap)?;
        }
        for arg in &self.args {
            writeln!(f, "    arg {}", arg)?;
        }
        for local in &self.locals {
            writeln!(f, "    local {}", local)?;
        }
        for instr in &self.body {
            writeln!(f, "    {}", instr)?;
        }
        writeln!(f, "}}")
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for g in &self.globals {
            writeln!(f, "global {}", g)?;
        }
        if !self.globals.is_empty() {
            writeln!(f)?;
        }
        writeln!(f, "entry {}", self.entry)?;
        writeln!(f)?;
        // Sort functions for deterministic output
        let mut labels: Vec<_> = self.functions.keys().collect();
        labels.sort();
        for label in labels {
            write!(f, "{}", self.functions.get(label).unwrap())?;
            writeln!(f)?;
        }
        Ok(())
    }
}
