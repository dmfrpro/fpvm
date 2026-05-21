# Codegen Specification

## 1. Top-level execution

Language F has no user-defined `main`.

A program is a sequence of top-level expressions. Codegen wraps them into an internal function:

```
__top_level
```

Bytecode starts with:

```
entry __top_level
```

The VM begins execution from this function.

---

## 3. Symbol table

The symbol table is produced before codegen and describes all declared names, scopes, functions, arguments, locals, globals, and captures.

See the `symbol_table` module for concrete types:
- `Symbol`
- `Scope`
- `FunctionInfo`

Each symbol has a unique VM label:
```
x      -> global_x_0
a      -> arg_a_1
tmp    -> local_tmp_2
sum    -> func_sum_3
```

Codegen uses these labels in bytecode instructions.

---

## 4. Captures

A capture is an external non-global variable used inside a nested function or lambda.

Example:

```
(func outer (x)
    (func inner (y)
        (plus x y)
    )
)
```

Here `inner` captures `x`.

Captures are stored in `FunctionInfo.captures`.

A lambda does not always capture variables:

```
(lambda (x)
    (plus x 1)
)
```

Here `x` is the lambda argument, so captures are empty.

---

## 5. Bytecode structure

A bytecode program contains:

```
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

Example:

```
global global_x_0

entry __top_level

func __top_level {
    loadint 1
    setglobal global_x_0
    ret
}
```

---

## 6. Stack model

The VM is stack-based.

Example:

```
loadint 1
loadint 2
add
```

Execution:

```
stack = []
loadint 1  -> [1]
loadint 2  -> [1, 2]
add        -> [3]
```

Operations take arguments from the stack and push results back.

---

## 7. Instructions

### Constants

```
loadnull
loadint <value>
loadreal <value>
loadbool <value>
loadatom <name>
loadfunc <function_label>
makelist <n>
```

### Variables

```
loadlocal <var_label>
loadarg <var_label>
loadcapture <var_label>
loadglobal <var_label>

setlocal <var_label>
setarg <var_label>
setcapture <var_label>
setglobal <var_label>

```
`load*` instructions load a variable value onto the stack.

`set*` instructions store the top stack value and rempve it from the stack.

Example:
```
(setq x 1)
loadint 1
setglobal global_x_0
```
After setglobal, the stack doesn't contain 1.

set* consumes the value, the stack would became empty, then to return value we need to load this value from storage:
```
loadint 1
setglobal global_x_0
loadglobal global_x_0
ret
```

### Arithmetic

```
add
sub
mul
div
mod
```

### Comparisons

```
eq
neq
less
leq
greater
geq
```

### Control flow

```
<label>:
jump <label>
condjump <label>
```

`condjump` pops a boolean and jumps if it is true.

### Calls

```
call <function_label>
callstack <argc>
```

`call` is used for statically known functions.
`callstack` is used when the function value is computed at runtime.


### Stack

```
pop
```

Used to discard intermediate expression results.

### Return

```
ret
```

`ret` expects exactly one value on the current frame stack. It removes the current frame and transfers this value to the caller frame stack. If the current frame is the entry frame, this value becomes the program result.

---

## 8. Codegen rules

### Literals

```
null     -> loadnull
true     -> loadbool true
false    -> loadbool false
1        -> loadint 1
1.5      -> loadreal 1.5
```

### Identifier

Codegen looks up the identifier in the current scope:

```
Global symbol              -> loadglobal
Argument of current func   -> loadarg
Local of current func      -> loadlocal
Symbol from outer function -> loadcapture
Function symbol            -> loadfunc
```

### `setq`

```
(setq x expr)
```

Generates:

```
<compile expr>
set* <x_label>
```

The concrete `set*` instruction depends on the resolved symbol (set`local`, set`capture`, set`arg`, set`global`).

### Sequence

Only the last expression result remains on the stack.

```
<compile expr1>
pop
<compile expr2>
pop
<compile expr3>
```

### Builtins

```
plus      -> add
minus     -> sub
times     -> mul
divide    -> div
mod       -> mod

equal     -> eq
nonequal  -> neq
less      -> less
lesseq    -> leq
greater   -> greater
greatereq -> geq
```

### `func`

Creates a named function symbol and a bytecode function.

As expression, it loads the function value:

```
loadfunc <function_label>
```

### `lambda`

Creates an anonymous function.

As expression:

```
loadfunc <lambda_label>
```

### Function call

```
(f arg1 arg2)
```

If `f` is a known function symbol:

```
<compile arg1>
<compile arg2>
call <function_label>
```

Otherwise, callstack is used when the target function is computed at runtime and stored on the stack:

```
<compile f>
<compile arg1>
<compile arg2>
callstack 2
```

### `cond`

```
<compile condition>
condjump then_label

<compile else>
jump end_label

then_label:
<compile then>

end_label:
```

If `else` is absent, generate `loadnull`. Because every expression must leave exactly one value on the stack.

### `while`

```
while_start:
<compile condition>
condjump while_body
jump while_end

while_body:
<compile body>
pop
jump while_start

while_end:
loadnull
```

### `return`

```
<compile expr>
ret
```

### `break`

```
jump <nearest_while_end_label>
```

### `quote`

Quote returns data without evaluation.

```
'x
```

```
loadatom x
```

```
'(plus 1 2)
```

```
loadatom plus
loadint 1
loadint 2
makelist 3
```

---
