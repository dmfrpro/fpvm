# Codegen Specification

## 1. Top-level execution

Language F has no user-defined `main`.

A program is a sequence of top-level expressions. Codegen wraps them into
an internal function:

```text
__top_level
```text

Bytecode starts with:

```text
entry __top_level
```text

The VM begins execution from this function.

---

## 3. Symbol table

The symbol table is produced before codegen and describes all declared names,
scopes, functions, arguments, locals, globals, and captures.

See the `symbol_table` module for concrete types:
- `Symbol`
- `Scope`
- `FunctionInfo`

Each symbol has a unique VM label:
```text
x      -> global_x_0
a      -> arg_a_1
tmp    -> local_tmp_2
sum    -> func_sum_3
```text

Codegen uses these labels in bytecode instructions.

---

## 4. Captures

A capture is an external non-global variable used inside a nested function or lambda.

Example:

```text
(func outer (x)
    (func inner (y)
        (plus x y)
    )
)
```text

Here `inner` captures `x`.

Captures are stored in `FunctionInfo.captures`.

A lambda does not always capture variables:

```text
(lambda (x)
    (plus x 1)
)
```text

Here `x` is the lambda argument, so captures are empty.

---

## 5. Bytecode structure

A bytecode program contains:

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
```text

Example:

```text
global global_x_0

entry __top_level

func __top_level {
    loadint 1
    setglobal global_x_0
    ret
}
```text

---

## 6. Stack model

The VM is stack-based.

Example:

```text
loadint 1
loadint 2
add
```text

Execution:

```text
stack = []
loadint 1  -> [1]
loadint 2  -> [1, 2]
add        -> [3]
```text

Operations take arguments from the stack and push results back.

---

## 7. Instructions

### Constants

```text
loadnull
loadint <value>
loadreal <value>
loadbool <value>
loadatom <name>
loadfunc <function_label>
makelist <n>
```text

### Variables

```text
loadlocal <var_label>
loadarg <var_label>
loadcapture <var_label>
loadglobal <var_label>

setlocal <var_label>
setarg <var_label>
setcapture <var_label>
setglobal <var_label>

```text
`load*` instructions load a variable value onto the stack.

`set*` instructions store the top stack value and rempve it from the stack.

Example:
```text
(setq x 1)
loadint 1
setglobal global_x_0
```text
After setglobal, the stack doesn't contain 1.

set* consumes the value, the stack would became empty, then to return value
we need to load this value from storage:
```text
loadint 1
setglobal global_x_0
loadglobal global_x_0
ret
```text

### Arithmetic

```text
add
sub
mul
div
mod
```text

### Comparisons

```text
eq
neq
less
leq
greater
geq
```text

### Control flow

```text
<label>:
jump <label>
condjump <label>
```text

`condjump` pops a boolean and jumps if it is true.

### Calls

```text
call <function_label>
callstack <argc>
```text

`call` is used for statically known functions.
`callstack` is used when the function value is computed at runtime.


### Stack

```text
pop
```text

Used to discard intermediate expression results.

### Return

```text
ret
```text

`ret` expects exactly one value on the current frame stack. It removes the
current frame and transfers this value to the caller frame stack. If the
current frame is the entry frame, this value becomes the program result.

---

## 8. Codegen rules

### Literals

```text
null     -> loadnull
true     -> loadbool true
false    -> loadbool false
1        -> loadint 1
1.5      -> loadreal 1.5
```text

### Identifier

Codegen looks up the identifier in the current scope:

```text
Global symbol              -> loadglobal
Argument of current func   -> loadarg
Local of current func      -> loadlocal
Symbol from outer function -> loadcapture
Function symbol            -> loadfunc
```text

### `setq`

```text
(setq x expr)
```text

Generates:

```text
<compile expr>
set* <x_label>
```text

The concrete `set*` instruction depends on the resolved symbol
(set`local`, set`capture`, set`arg`, set`global`).

### Sequence

Only the last expression result remains on the stack.

```text
<compile expr1>
pop
<compile expr2>
pop
<compile expr3>
```text

### Builtins

```text
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
```text

### `func`

Creates a named function symbol and a bytecode function.

As expression, it loads the function value:

```text
loadfunc <function_label>
```text

### `lambda`

Creates an anonymous function.

As expression:

```text
loadfunc <lambda_label>
```text

### Function call

```text
(f arg1 arg2)
```text

If `f` is a known function symbol:

```text
<compile arg1>
<compile arg2>
call <function_label>
```text

Otherwise, callstack is used when the target function is computed at runtime
and stored on the stack:

```text
<compile f>
<compile arg1>
<compile arg2>
callstack 2
```text

### `cond`

```text
<compile condition>
condjump then_label

<compile else>
jump end_label

then_label:
<compile then>

end_label:
```text

If `else` is absent, generate `loadnull`. Because every expression must leave
exactly one value on the stack.

### `while`

```text
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
```text

### `return`

```text
<compile expr>
ret
```text

### `break`

```text
jump <nearest_while_end_label>
```text

### `quote`

Quote returns data without evaluation.

```text
'x
```text

```text
loadatom x
```text

```text
'(plus 1 2)
```text

```text
loadatom plus
loadint 1
loadint 2
makelist 3
```text

---
