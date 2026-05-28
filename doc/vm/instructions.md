# Instruction Reference

## Constants

### `loadnull`

Pushes `Null` onto the stack.

### `loadint <value>`

Pushes an integer onto the stack.

```text
loadint 42
```

### `loadreal <value>`

Pushes a floating-point number onto the stack.

```text
loadreal 3.14
```

### `loadbool <value>`

Pushes a boolean (`true` or `false`) onto the stack.

```text
loadbool true
```

### `loadatom <name>`

Pushes an atom onto the stack.

```text
loadatom plus
```

### `loadfunc <label>`

Pushes a function reference onto the stack.
If the function has captures, a `Closure` is pushed instead.

```text
loadfunc func_sum_0
```

### `makelist <n>`

Pops `n` values from the stack, builds a list, and pushes it.
If `n` is 0, pushes `Null`.

```text
loadint 1
loadint 2
loadint 3
makelist 3
```

Result: `[1, 2, 3]`

---

## Variables

### `loadlocal <label>`

### `loadarg <label>`

### `loadcapture <label>`

### `loadglobal <label>`

Load a variable value onto the stack.

```text
loadlocal local_x_0
```

### `setlocal <label>`

### `setarg <label>`

### `setcapture <label>`

### `setglobal <label>`

Pop the top stack value and store it into the variable.

```text
loadint 10
setglobal global_x_0
```

After `set*`, the value is removed from the stack.

---

## Arithmetic

All arithmetic instructions pop two values and push the result.

| Instruction | Operation |
|-------------|-----------|
| `add`       | a + b     |
| `sub`       | a - b     |
| `mul`       | a * b     |
| `div`       | a / b     |
| `mod`       | a % b     |

### Type rules

- If **both** arguments are `Int`, the result is `Int`.
- If **either** argument is `Real`, both are promoted to `Real`
  and the result is `Real`.
- Any other combination causes a runtime type error.

```text
loadint 5
loadreal 1.3
add
```

Result: `6.3` (Real)

```text
loadint 7
loadint 2
div
```

Result: `3` (Int)

```text
loadreal 7.0
loadint 2
div
```

Result: `3.5` (Real)

---

## Comparisons

All comparison instructions pop two values and push a `Bool`.

| Instruction | Operation |
|-------------|-----------|
| `eq`        | a == b    |
| `neq`       | a != b    |
| `less`      | a < b     |
| `leq`       | a <= b    |
| `greater`   | a > b     |
| `geq`       | a >= b    |

Comparisons work on any pair of values that implement
`PartialEq` / `PartialOrd`.

---

## Control Flow

### `<label>:`

Defines a label for jumps. Labels are local to the function.

```text
loop_start:
```

### `jump <label>`

Unconditional jump to `label`.

```text
jump loop_start
```

### `condjump <label>`

Pops a boolean from the stack. Jumps to `label` if it is `true`.

```text
loadbool false
condjump else_label
```

---

## Calls

### `call <label>`

Calls a statically known function. Arguments are taken from the stack.

```text
loadint 1
loadint 2
call func_plus_0
```

### `callstack <argc>`

Calls a function value that is computed at runtime.
The function value is on top of the stack, followed by `argc`
arguments.

```text
loadfunc func_plus_0
loadint 1
loadint 2
callstack 2
```

`callstack` also handles:

- `Closure` values
- `Atom` values (dispatches to built-in operations via `eval`)

---

## Stack

### `pop`

Discards the top stack value.

---

## Output

### `stdout`

Pops the top stack value and prints it to stdout.

```text
loadint 42
stdout
```

Output:

```text
42
```

---

## Return

### `ret`

Pops the top stack value, destroys the current frame, and pushes
the value to the caller's stack. If there is no caller, the value
becomes the program result.

If the function body ends without `ret`, the VM automatically pushes
`Null` and returns.

---

## Built-ins

These instructions correspond to language built-ins.

### `head`

Pops a list and pushes its first element. Returns `Null` for empty
list or `Null`.

### `tail`

Pops a list and pushes all elements except the first.
Returns `Null` for empty or single-element list.

### `cons`

Pops a list and a value, then pushes a new list with the value
prepended.

### `isnull`

Pops a value and pushes `true` if it is `Null`.

### `length`

Pops a list and pushes its length as an `Int`. `Null` has length 0.

### `or`

Pops two booleans and pushes their logical OR.

### `not`

Pops a boolean and pushes its logical negation.

### `islist`

Pops a value and pushes `true` if it is a list.

### `eval`

Pops a value and evaluates it:

- `Atom` -> looks up variable or function
- `List` -> evaluates head and applies it to arguments
- Other values -> returned as-is

---

## Error Messages

| Error                        | Cause                               |
|------------------------------|-------------------------------------|
| `Unknown function`           | `call` or `loadfunc` is invalid     |
| `Unknown label`              | `jump` target does not exist        |
| `Undefined variable`         | `load*` for missing variable        |
| `Type error in arithmetic`   | Arithmetic on incompatible types    |
| `Stack underflow`            | Not enough values on the stack      |
| `Step limit exceeded`        | Infinite loop or long execution     |
| `Argument count mismatch`    | Wrong number of arguments to `call` |
