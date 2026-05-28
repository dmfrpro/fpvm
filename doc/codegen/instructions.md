# Instruction Reference

## Arithmetic Operations

These commands take two items from the stack and put the result
on top of the stack.

- `add` — returns `int` if both arguments are ints, otherwise `real`
- `sub` — returns `int` if both arguments are ints, otherwise `real`
- `mul` — returns `int` if both arguments are ints, otherwise `real`
- `div` — returns `int` if both arguments are ints, otherwise `real`
- `mod` — returns `int` if both arguments are ints, otherwise `real`

## Boolean Operations

Only take `real`, `int`, `bool`, `list` (only for `eq` and `neq`)
as arguments. Puts `bool` on top of the stack.

- `eq`
- `neq`
- `less`
- `leq`
- `greater`
- `geq`

## Jumps

```text
jump <label>
condjump <label>
<label>:
```

`condjump` takes a `bool` from the top of the stack and jumps on
`true`.

## Functions

```text
func <func_label> {
    capture <var_label>
    arg <var_label>
    local <var_label>

    <body>
}
```

- `capture` — interpreter binds address of captures to the function
- `args` — function arguments
- `locals` — local variables
- Body: if no explicit return, returns `null`

### Variable Instructions

- `loadlocal <var_label>` — load value of variable to top of stack
- `loadcapture <var_label>`
- `loadarg <var_label>`
- `loadglobal <var_label>`
- `setlocal <var_label>` — sets local variable to top of stack value
- `setcapture <var_label>`
- `setarg <var_label>`
- `setglobal <var_label>`

### Call Instructions

- `call <func_label>` — calls function with args on top of stack
- `callstack <argc>` — calls function from top of stack with args
- `ret` — returns single element from stack

## Return

```text
ret
```

Returns the top stack value from the current function.

## Globals

```text
global <var_label>
```

## Constants

Loads values or addresses to the top of the stack.

- `loadnull`
- `loadint <value>`
- `loadreal <value>`
- `loadbool <value>`
- `loadfunc <func_label>` — finds function address and puts it on stack
- `makelist <n>` — takes `n` args from stack and puts a single list

## StdLib

Separate file with bytecode for stdlib. Metadata for the file should
contain all labels and be loaded for execution.

## Virtual Function

```text
createvfunc <new_func_label> <base_func_label>
```

- Its own `new_func_label`
- Label on base function
- Captured args

```text
setcapturearg <func_label> <var_label>
setcaptureargconst <real,int,bool,list>
```
