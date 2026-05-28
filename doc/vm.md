# Running the VM from the Terminal

The `fpvm` binary reads bytecode from a file and executes it.

## Examples

1. Run bytecode file

    ```sh
    fpvm program.bytecode
    ```

2. Run via cargo

    ```sh
    cargo run --bin fpvm -- program.bytecode
    ```

3. Pipe bytecode from stdin

    ```sh
    cat program.bytecode | fpvm
    ```

## Output

The VM prints the result of the `entry` function to stdout.

If the program uses `stdout` instructions, their output is printed
line by line.

Runtime errors are printed to stderr:

```text
Runtime error: Type error in arithmetic
```
