# VM Architecture

## Overview

The VM is a stack-based interpreter that executes bytecode programs.

It consists of three main components:

- `bytecode` - parser and representation of bytecode programs
- `value` - runtime value types
- `vm` - executor with call stack and instruction loop

---

## Value Types

The VM operates on a single `Value` enum:

```rust
pub enum Value {
    Null,
    Int(i64),
    Real(f64),
    Bool(bool),
    Atom(String),
    List(Vec<Value>),
    Func(String),
    Closure { label: String, captures: Vec<Value> },
}
```

| Variant   | Description                                |
|-----------|--------------------------------------------|
| `Null`    | Empty value and empty list                 |
| `Int`     | 64-bit signed integer                      |
| `Real`    | 64-bit floating point                      |
| `Bool`    | Boolean (`true` / `false`)                 |
| `Atom`    | Symbolic name (e.g. `'plus`)               |
| `List`    | Heterogeneous list of values               |
| `Func`    | Reference to a bytecode function           |
| `Closure` | Function reference with captured variables |

---

## Stack Model

Each function call creates a new frame with its own operand stack.

Instructions push and pop values from the current frame's stack:

```text
loadint 1
loadint 2
add
```

Execution:

```text
stack = []
loadint 1  -> [1]
loadint 2  -> [1, 2]
add        -> [3]
```

---

## Call Frame

Each active function call has a frame containing:

- `function_label` - name of the executing function
- `locals` - local variables map
- `args` - argument variables map
- `captures` - captured variables map
- `stack` - operand stack
- `pc` - program counter (instruction index)

Frames are pushed on `call` and popped on `ret`.

---

## Program Structure

A bytecode program consists of:

```text
global <var_label>
entry <function_label>

func <function_label> {
    capture <var_label>
    arg <var_label>
    local <var_label>

    <instructions>
    ret
}
```

The VM starts execution from the `entry` function.

---

## Step Limit

To prevent infinite loops, the VM has a step limit of **100 000**
instructions. If exceeded, execution aborts with:

```text
Step limit exceeded
```
