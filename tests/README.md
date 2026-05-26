# Run and get symbol table output

```
cargo run --bin dump_sb_tool -- tests/table_check/code
```
---

Problems:

1. Invalid program
```
(setq x 1)
(setq x 1)
(setq x 1)
(setq x 1)
```

2. return stmt in prog body
`ReturnOutsideFunction`
return works only if prog is wrapped in `func` special form