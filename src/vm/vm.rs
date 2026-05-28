use std::collections::HashMap;

use super::bytecode::{Instruction, Program};
use super::value::Value;

#[derive(Debug, Clone)]
struct Frame {
    function_label: String,
    locals: HashMap<String, Value>,
    args: HashMap<String, Value>,
    captures: HashMap<String, Value>,
    stack: Vec<Value>,
    pc: usize,
}

pub struct Vm {
    program: Program,
    call_stack: Vec<Frame>,
    globals: HashMap<String, Value>,
    output: Vec<String>,
    labels: HashMap<String, HashMap<String, usize>>,
    result: Option<Value>,
}

impl Vm {
    pub fn new(program: Program) -> Self {
        let mut labels: HashMap<String, HashMap<String, usize>> = HashMap::new();
        for (label, func) in &program.functions {
            let mut func_labels = HashMap::new();
            for (idx, instr) in func.body.iter().enumerate() {
                if let Instruction::Label(l) = instr {
                    func_labels.insert(l.clone(), idx);
                }
            }
            labels.insert(label.clone(), func_labels);
        }

        let mut globals = HashMap::new();
        for g in &program.globals {
            globals.insert(g.clone(), Value::Null);
        }

        Self {
            program,
            call_stack: Vec::new(),
            globals,
            output: Vec::new(),
            labels,
            result: None,
        }
    }

    pub fn run(&mut self) -> Result<Value, String> {
        let entry = self.program.entry.clone();
        self.call_function(entry, Vec::new(), Vec::new())?;

        let mut step = 0;
        while !self.call_stack.is_empty() {
            if let Some(ref val) = self.result {
                return Ok(val.clone());
            }

            let frame_idx = self.call_stack.len() - 1;
            let func_label = self.call_stack[frame_idx].function_label.clone();

            let func = match self.program.functions.get(&func_label) {
                Some(f) => f.clone(),
                None => return Err(format!("Unknown function: {}", func_label)),
            };

            let pc = self.call_stack[frame_idx].pc;

            if pc >= func.body.len() {
                self.call_stack[frame_idx].stack.push(Value::Null);
                self.do_ret()?;
                continue;
            }

            let instr = &func.body[pc];
            self.call_stack[frame_idx].pc += 1;

            step += 1;
            if step > 100_000 {
                return Err("Step limit exceeded".to_string());
            }

            self.execute_instruction(instr, &func_label)?;
        }

        Ok(Value::Null)
    }

    fn current_frame(&mut self) -> &mut Frame {
        let idx = self.call_stack.len() - 1;
        &mut self.call_stack[idx]
    }

    fn push(&mut self, val: Value) {
        self.current_frame().stack.push(val);
    }

    fn pop(&mut self) -> Result<Value, String> {
        let frame = self.current_frame();
        let pc = frame.pc;
        let label = frame.function_label.clone();
        frame
            .stack
            .pop()
            .ok_or_else(|| format!("Stack underflow in {} at pc {}", label, pc))
    }

    fn lookup_current(&self, label: &str) -> Result<Value, String> {
        let frame = self.call_stack.last().unwrap();
        if let Some(v) = frame.locals.get(label) {
            return Ok(v.clone());
        }
        if let Some(v) = frame.args.get(label) {
            return Ok(v.clone());
        }
        if let Some(v) = frame.captures.get(label) {
            return Ok(v.clone());
        }
        if let Some(v) = self.globals.get(label) {
            return Ok(v.clone());
        }
        Err(format!("Lookup failed: {}", label))
    }

    fn resolve_label(&self, func_label: &str, label: &str) -> Result<usize, String> {
        self.labels
            .get(func_label)
            .and_then(|m| m.get(label))
            .copied()
            .ok_or_else(|| format!("Unknown label: {}", label))
    }

    fn execute_instruction(&mut self, instr: &Instruction, func_label: &str) -> Result<(), String> {
        match instr {
            Instruction::LoadNull => self.push(Value::Null),
            Instruction::LoadInt(v) => self.push(Value::Int(*v)),
            Instruction::LoadReal(v) => self.push(Value::Real(*v)),
            Instruction::LoadBool(v) => self.push(Value::Bool(*v)),
            Instruction::LoadAtom(v) => self.push(Value::Atom(v.clone())),
            Instruction::LoadFunc(label) => {
                let func = self.program.functions.get(label).cloned();
                if let Some(func) = func {
                    if !func.captures.is_empty() {
                        let mut captures = Vec::new();
                        for cap_label in &func.captures {
                            captures.push(self.lookup_current(cap_label)?);
                        }
                        self.push(Value::Closure {
                            label: label.clone(),
                            captures,
                        });
                    } else {
                        self.push(Value::Func(label.clone()));
                    }
                } else {
                    return Err(format!("loadfunc: unknown function {}", label));
                }
            }
            Instruction::MakeList(n) => {
                if *n == 0 {
                    self.push(Value::Null);
                } else {
                    let mut items = Vec::new();
                    for _ in 0..*n {
                        items.push(self.pop()?);
                    }
                    items.reverse();
                    self.push(Value::List(items));
                }
            }
            Instruction::LoadLocal(label) => {
                let val = self
                    .current_frame()
                    .locals
                    .get(label)
                    .cloned()
                    .ok_or_else(|| format!("Undefined local: {}", label))?;
                self.push(val);
            }
            Instruction::LoadArg(label) => {
                let val = self
                    .current_frame()
                    .args
                    .get(label)
                    .cloned()
                    .ok_or_else(|| format!("Undefined arg: {}", label))?;
                self.push(val);
            }
            Instruction::LoadCapture(label) => {
                let val = self
                    .current_frame()
                    .captures
                    .get(label)
                    .cloned()
                    .ok_or_else(|| format!("Undefined capture: {}", label))?;
                self.push(val);
            }
            Instruction::LoadGlobal(label) => {
                let val = self
                    .globals
                    .get(label)
                    .cloned()
                    .ok_or_else(|| format!("Undefined global: {}", label))?;
                self.push(val);
            }
            Instruction::SetLocal(label) => {
                let val = self.pop()?;
                self.current_frame().locals.insert(label.clone(), val);
            }
            Instruction::SetArg(label) => {
                let val = self.pop()?;
                self.current_frame().args.insert(label.clone(), val);
            }
            Instruction::SetCapture(label) => {
                let val = self.pop()?;
                self.current_frame().captures.insert(label.clone(), val);
            }
            Instruction::SetGlobal(label) => {
                let val = self.pop()?;
                self.globals.insert(label.clone(), val);
            }
            Instruction::Add => self.binop_num(|a, b| a + b, |a, b| a + b)?,
            Instruction::Sub => self.binop_num(|a, b| a - b, |a, b| a - b)?,
            Instruction::Mul => self.binop_num(|a, b| a * b, |a, b| a * b)?,
            Instruction::Div => self.binop_num(|a, b| a / b, |a, b| a / b)?,
            Instruction::Mod => self.binop_num(|a, b| a % b, |a, b| a % b)?,
            Instruction::Eq => self.binop_cmp(|a, b| Value::Bool(a == b))?,
            Instruction::Neq => self.binop_cmp(|a, b| Value::Bool(a != b))?,
            Instruction::Less => self.binop_cmp(|a, b| Value::Bool(a < b))?,
            Instruction::Leq => self.binop_cmp(|a, b| Value::Bool(a <= b))?,
            Instruction::Greater => self.binop_cmp(|a, b| Value::Bool(a > b))?,
            Instruction::Geq => self.binop_cmp(|a, b| Value::Bool(a >= b))?,
            Instruction::Label(_) => {}
            Instruction::Jump(label) => {
                let target = self.resolve_label(func_label, label)?;
                self.current_frame().pc = target + 1;
            }
            Instruction::CondJump(label) => {
                let cond = self.pop()?;
                if matches!(cond, Value::Bool(true)) {
                    let target = self.resolve_label(func_label, label)?;
                    self.current_frame().pc = target + 1;
                }
            }
            Instruction::Call(label) => {
                let func = self
                    .program
                    .functions
                    .get(label)
                    .cloned()
                    .ok_or_else(|| format!("Unknown function: {}", label))?;
                let mut args = Vec::new();
                for _ in 0..func.args.len() {
                    args.push(self.pop()?);
                }
                args.reverse();

                let mut captures = Vec::new();
                if !func.captures.is_empty() {
                    for cap_label in &func.captures {
                        captures.push(self.lookup_current(cap_label)?);
                    }
                }

                self.call_function(label.clone(), args, captures)?;
            }
            Instruction::CallStack { argc } => {
                let mut args = Vec::new();
                for _ in 0..*argc {
                    args.push(self.pop()?);
                }
                args.reverse();

                let callee = self.pop()?;
                match callee {
                    Value::Func(label) => {
                        self.call_function(label, args, Vec::new())?;
                    }
                    Value::Closure { label, captures } => {
                        self.call_function(label, args, captures)?;
                    }
                    Value::Atom(name) => {
                        let result = self.apply_value(&Value::Atom(name), &args)?;
                        self.push(result);
                    }
                    _ => return Err(format!("callstack: not a function: {:?}", callee)),
                }
            }
            Instruction::Pop => {
                self.pop()?;
            }
            Instruction::Stdout => {
                let val = self.pop()?;
                self.output.push(format!("{}", val));
            }
            Instruction::Ret => {
                self.do_ret()?;
            }
            Instruction::Head => self.builtin_head()?,
            Instruction::Tail => self.builtin_tail()?,
            Instruction::Cons => self.builtin_cons()?,
            Instruction::IsNull => self.builtin_isnull()?,
            Instruction::Length => self.builtin_length()?,
            Instruction::Or => self.builtin_or()?,
            Instruction::Not => self.builtin_not()?,
            Instruction::IsList => self.builtin_islist()?,
            Instruction::Eval => self.builtin_eval()?,
        }
        Ok(())
    }

    fn binop_num<F, G>(&mut self, int_op: F, real_op: G) -> Result<(), String>
    where
        F: Fn(i64, i64) -> i64,
        G: Fn(f64, f64) -> f64,
    {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(apply_numeric(a, b, int_op, real_op)?);
        Ok(())
    }

    fn binop_cmp<F>(&mut self, op: F) -> Result<(), String>
    where
        F: Fn(&Value, &Value) -> Value,
    {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(op(&a, &b));
        Ok(())
    }

    fn call_function(
        &mut self,
        label: String,
        args: Vec<Value>,
        captures: Vec<Value>,
    ) -> Result<(), String> {
        let func = self
            .program
            .functions
            .get(&label)
            .cloned()
            .ok_or_else(|| format!("Unknown function: {}", label))?;

        if args.len() != func.args.len() {
            return Err(format!(
                "Argument count mismatch for {}: expected {}, got {}",
                label,
                func.args.len(),
                args.len()
            ));
        }

        let mut arg_map = HashMap::new();
        for (i, val) in args.iter().enumerate() {
            arg_map.insert(func.args[i].clone(), val.clone());
        }

        let mut capture_map = HashMap::new();
        for (i, val) in captures.iter().enumerate() {
            capture_map.insert(func.captures[i].clone(), val.clone());
        }

        self.call_stack.push(Frame {
            function_label: label,
            locals: HashMap::new(),
            args: arg_map,
            captures: capture_map,
            stack: Vec::new(),
            pc: 0,
        });
        Ok(())
    }

    fn do_ret(&mut self) -> Result<(), String> {
        let val = self.pop()?;
        self.call_stack.pop();
        if let Some(frame) = self.call_stack.last_mut() {
            frame.stack.push(val);
        } else {
            self.result = Some(val);
        }
        Ok(())
    }

    fn builtin_head(&mut self) -> Result<(), String> {
        match self.pop()? {
            Value::Null => self.push(Value::Null),
            Value::List(items) => {
                if items.is_empty() {
                    self.push(Value::Null);
                } else {
                    self.push(items[0].clone());
                }
            }
            _ => return Err("head: not a list".to_string()),
        }
        Ok(())
    }

    fn builtin_tail(&mut self) -> Result<(), String> {
        match self.pop()? {
            Value::Null => self.push(Value::Null),
            Value::List(items) => {
                if items.len() <= 1 {
                    self.push(Value::Null);
                } else {
                    self.push(Value::List(items[1..].to_vec()));
                }
            }
            _ => return Err("tail: not a list".to_string()),
        }
        Ok(())
    }

    fn builtin_cons(&mut self) -> Result<(), String> {
        let list = self.pop()?;
        let head = self.pop()?;
        match list {
            Value::Null => {
                self.push(Value::List(vec![head]));
            }
            Value::List(mut items) => {
                items.insert(0, head);
                self.push(Value::List(items));
            }
            _ => return Err("cons: not a list".to_string()),
        }
        Ok(())
    }

    fn builtin_isnull(&mut self) -> Result<(), String> {
        let val = self.pop()?;
        self.push(Value::Bool(matches!(val, Value::Null)));
        Ok(())
    }

    fn builtin_length(&mut self) -> Result<(), String> {
        match self.pop()? {
            Value::Null => self.push(Value::Int(0)),
            Value::List(items) => self.push(Value::Int(items.len() as i64)),
            _ => return Err("length: not a list".to_string()),
        }
        Ok(())
    }

    fn builtin_or(&mut self) -> Result<(), String> {
        let b = self.pop()?;
        let a = self.pop()?;
        let result = matches!(a, Value::Bool(true)) || matches!(b, Value::Bool(true));
        self.push(Value::Bool(result));
        Ok(())
    }

    fn builtin_not(&mut self) -> Result<(), String> {
        let a = self.pop()?;
        let result = !matches!(a, Value::Bool(true));
        self.push(Value::Bool(result));
        Ok(())
    }

    fn builtin_islist(&mut self) -> Result<(), String> {
        let a = self.pop()?;
        let result = matches!(a, Value::List(_));
        self.push(Value::Bool(result));
        Ok(())
    }

    fn builtin_eval(&mut self) -> Result<(), String> {
        let val = self.pop()?;
        let result = self.eval_value(&val)?;
        self.push(result);
        Ok(())
    }

    fn eval_value(&mut self, val: &Value) -> Result<Value, String> {
        match val {
            Value::Atom(name) => {
                if self.program.functions.contains_key(name) {
                    return Ok(Value::Func(name.clone()));
                }
                self.lookup_current(name)
                    .or_else(|_| Ok(Value::Atom(name.clone())))
            }
            Value::List(items) => {
                if items.is_empty() {
                    return Ok(Value::Null);
                }
                let head = self.eval_value(&items[0])?;
                let mut args = Vec::new();
                for i in 1..items.len() {
                    args.push(self.eval_value(&items[i])?);
                }
                self.apply_value(&head, &args)
            }
            _ => Ok(val.clone()),
        }
    }

    fn apply_value(&mut self, head: &Value, args: &[Value]) -> Result<Value, String> {
        match head {
            Value::Func(label) => self.run_eval_call(label.clone(), args.to_vec(), Vec::new()),
            Value::Closure { label, captures } => {
                self.run_eval_call(label.clone(), args.to_vec(), captures.clone())
            }
            Value::Atom(name) => match name.as_str() {
                "plus" | "minus" | "times" | "divide" | "mod" => {
                    if args.len() != 2 {
                        return Err(format!("{} expects 2 args", name));
                    }
                    let a = args[0].clone();
                    let b = args[1].clone();
                    let op_name = name.as_str();
                    apply_numeric(
                        a,
                        b,
                        |a, b| match op_name {
                            "plus" => a + b,
                            "minus" => a - b,
                            "times" => a * b,
                            "divide" => a / b,
                            "mod" => a % b,
                            _ => unreachable!(),
                        },
                        |a, b| match op_name {
                            "plus" => a + b,
                            "minus" => a - b,
                            "times" => a * b,
                            "divide" => a / b,
                            "mod" => a % b,
                            _ => unreachable!(),
                        },
                    )
                }
                "equal" | "nonequal" | "less" | "lesseq" | "greater" | "greatereq" => {
                    if args.len() != 2 {
                        return Err(format!("{} expects 2 args", name));
                    }
                    let result = match name.as_str() {
                        "equal" => &args[0] == &args[1],
                        "nonequal" => &args[0] != &args[1],
                        "less" => args[0] < args[1],
                        "lesseq" => args[0] <= args[1],
                        "greater" => args[0] > args[1],
                        "greatereq" => args[0] >= args[1],
                        _ => unreachable!(),
                    };
                    Ok(Value::Bool(result))
                }
                "head" => {
                    if args.len() != 1 {
                        return Err("head expects 1 arg".to_string());
                    }
                    match &args[0] {
                        Value::List(items) if !items.is_empty() => Ok(items[0].clone()),
                        Value::List(_) => Ok(Value::Null),
                        _ => Err("head: not a list".to_string()),
                    }
                }
                "tail" => {
                    if args.len() != 1 {
                        return Err("tail expects 1 arg".to_string());
                    }
                    match &args[0] {
                        Value::List(items) if !items.is_empty() => {
                            Ok(Value::List(items[1..].to_vec()))
                        }
                        Value::List(_) => Ok(Value::Null),
                        _ => Err("tail: not a list".to_string()),
                    }
                }
                "cons" => {
                    if args.len() != 2 {
                        return Err("cons expects 2 args".to_string());
                    }
                    match &args[1] {
                        Value::List(items) => {
                            let mut new = items.clone();
                            new.insert(0, args[0].clone());
                            Ok(Value::List(new))
                        }
                        Value::Null => Ok(Value::List(vec![args[0].clone()])),
                        _ => Err("cons: not a list".to_string()),
                    }
                }
                "isnull" => {
                    if args.len() != 1 {
                        return Err("isnull expects 1 arg".to_string());
                    }
                    Ok(Value::Bool(matches!(args[0], Value::Null)))
                }
                "length" => {
                    if args.len() != 1 {
                        return Err("length expects 1 arg".to_string());
                    }
                    match &args[0] {
                        Value::List(items) => Ok(Value::Int(items.len() as i64)),
                        Value::Null => Ok(Value::Int(0)),
                        _ => Err("length: not a list".to_string()),
                    }
                }
                _ => Err(format!("eval: unknown function: {}", name)),
            },
            _ => Err(format!("eval: invalid function: {:?}", head)),
        }
    }

    fn run_eval_call(
        &mut self,
        label: String,
        args: Vec<Value>,
        captures: Vec<Value>,
    ) -> Result<Value, String> {
        let target_depth = self.call_stack.len();
        self.call_function(label, args, captures)?;

        let mut step = 0;
        loop {
            if self.call_stack.len() <= target_depth {
                let result = if self.call_stack.len() == target_depth {
                    self.pop().unwrap_or(Value::Null)
                } else {
                    self.result.take().unwrap_or(Value::Null)
                };
                return Ok(result);
            }

            let frame_idx = self.call_stack.len() - 1;
            let func_label = self.call_stack[frame_idx].function_label.clone();

            let func = match self.program.functions.get(&func_label) {
                Some(f) => f.clone(),
                None => return Err(format!("Unknown function: {}", func_label)),
            };

            let pc = self.call_stack[frame_idx].pc;

            if pc >= func.body.len() {
                self.call_stack[frame_idx].stack.push(Value::Null);
                self.do_ret()?;
                continue;
            }

            let instr = &func.body[pc];
            self.call_stack[frame_idx].pc += 1;

            step += 1;
            if step > 100_000 {
                return Err("Step limit exceeded".to_string());
            }

            self.execute_instruction(instr, &func_label)?;
        }
    }

    pub fn output(&self) -> &[String] {
        &self.output
    }

    pub fn take_output(&mut self) -> Vec<String> {
        std::mem::take(&mut self.output)
    }
}

fn apply_numeric<F, G>(a: Value, b: Value, int_op: F, real_op: G) -> Result<Value, String>
where
    F: Fn(i64, i64) -> i64,
    G: Fn(f64, f64) -> f64,
{
    match (a, b) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(int_op(a, b))),
        (Value::Int(a), Value::Real(b)) => Ok(Value::Real(real_op(a as f64, b))),
        (Value::Real(a), Value::Int(b)) => Ok(Value::Real(real_op(a, b as f64))),
        (Value::Real(a), Value::Real(b)) => Ok(Value::Real(real_op(a, b))),
        _ => Err("Type error in arithmetic".to_string()),
    }
}
