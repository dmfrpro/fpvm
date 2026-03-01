# Running the Syntax Analysis from the Terminal

The "syntax_bin" reads code from "stdin" and prints AST into "stdout"

## Examples

1. Pass code using echo

    ```sh
    # echo
    echo "(setq Atom2 '(plus -100 +2.4))" | cargo run --bin syntax_bin
    ```

2. Read from input file

    ```sh
    # stdin from file
    cat examples/arithmetic.fpvm | cargo run --bin syntax_bin
    ```
