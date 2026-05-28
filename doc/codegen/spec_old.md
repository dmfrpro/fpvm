# Language F - Semantic Specification (Draft)

## 1. Overview

F is an interpreted functional language based on S-expressions.
A program is a sequence of elements evaluated from first to the last
Element.

The result of a program is the result of evaluating the last element.

---

## 2. Program Execution

A program is a sequence of elements:

```text
Program = E1 E2 ... En
```

Execution:

- Evaluate elements in order
- Final result = eval(En)

---

## 3. Evaluation Model

Evaluation is defined by a function:

```text
eval(element, env) -> EvalResult
```

Where:

```text
EvalResult =
- Value
- Return(Value)
- Break
```

---

## 4. Basic Evaluation Rules

### 4.1 Literals

```text
eval(literal, env) = literal
```

---

### 4.2 Atoms

```text
eval(atom, env) =
- value bound to atom in env
- error if not found
```

---

### 4.3 Lists

```text
eval((f arg1 arg2 ...), env):
```

1. If f is a special form -> use special rules
2. Otherwise:
   - f_val = eval(f, env)
   - args = eval(arg1), eval(arg2), ... (left to right)
   - apply(f_val, args)

---

## 5. Function Application

```text
apply(f, args):
```

---

## 8. Environments

An **environment** is a stack of scopes.

**Scope** is Identifier -> Value

Rules:

- Lookup proceeds from the innermost scope outward.
- If a name is not found, evaluation fails.
- New scopes are created by: func, lambda, prog.
- Scopes are destroyed after exiting their construct.

---

## 7. Runtime Values

The language operates on the following types of values:

- Integer
- Real
- Boolean
- Null
- Atom
- List
- Function

Notes:

- Atoms may represent variables or symbolic values.
- Lists are ordered sequences of values.
- Functions are first-class values.

---

## 8. Special Forms

### 8.1 quote

Syntax:

```text
(quote x) or 'x
```

Semantics:

- Returns x without evaluation

---

### 8.2 setq

Syntax:

```text
(setq name expr)
```

Semantics:

1. name is not evaluated
2. value = eval(expr, env)
3. bind *name -> value* in current scope
4. return value

---

### 8.3 func

Syntax:

```text
(func name (param1, params2, ...) body)
```

Semantics:

1. Create function:
   - parameters = param1, params2, ...
   - body       = body
   - closure    = current environment
2. Bind *name -> function* in current scope
3. Return the function

---

### 8.4 lambda

Syntax:

```text
(lambda (param1, params2, ...) body)
```

Semantics:

- Returns a function value with:
  - parameters = param1, params2, ...
  - body
  - closure environment

---

### 8.5 prog

Syntax:

```text
(prog (var1, var2, ...) expr1 expr2 ...)
```

Semantics:

1. Create new scope
2. Initialize vars with null
3. Evaluate expressions sequentially
4. If *return* encountered -> propagate
5. Returns last expression result or null

---

### 8.6 cond

Syntax:

```text
(cond condition then [else])
```

Semantics:

1. Evaluate condition
2. If true -> evaluate then
3. Else:
   - if else exists -> evaluate else
   - otherwise return null

---

### 8.7 while

Syntax:

```text
(while condition body)
```

Semantics:

Loop:

1. Evaluate condition
2. If false -> stop
3. Evaluate body
   - if *break* -> exit loop
   - if *return* -> propagate
4. Repeat

Result: null

---

### 8.8 return

Syntax:

```text
(return expr)
```

Semantics:

- value = eval(expr)
- return Return(value)

---

### 8.9 break

Syntax:

```text
(break)
```

Semantics:

- return Break

---

### Built-in Function

- Validate argument count and types
- Execute operation
- Return result

### User-defined Function

Given Function(params, body, closure):

1. Create new scope
2. Bind params -> args
3. Evaluate body in new scope
4. If Return(v) -> return v
5. Otherwise return result

---

## 8. Control Flow Propagation

Rules:

- Return(value) propagates until function/prog boundary
- Break propagates until while

---
