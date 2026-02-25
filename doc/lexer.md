# Running the Lexer from the Terminal

The "lex_bin" reads code from "stdin" and prints tokens stream into "stdout"

## Examples

1. Pass code using echo

    ```sh
    # echo
    echo "(setq Atom2 '(plus -100 +2.4))" | cargo run --bin lex_bin
    ```

2. Read from input file

    ```sh
    # stdin from file
    cat examples/arithmetic.fpvm | cargo run --bin lex_bin
    ```
