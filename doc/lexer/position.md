# Position

## Overview

`Position` represents a location in the source code.
It abstracts raw byte offsets into human-readable coordinates to track coordinates of tokens and errors.

`Position` struct contains:
- `offset`  - zero-based byte position from the start of the source text. It is used for internal indexing and always corresponds to a valid utf-8 boundary.
- `line`    - 1-based line number
- `column`  - 1-based column number

---

## Invariants

- `line` >= 1
- `column` >= 1
- `offset` corresponds to a valid utf-8 boundary

---

## Example

Source code:
```   
(plus 10
    (times 2 5) 
)
```

Position of `p` in `plus`:
```rust
Position {
    offset: 1, // 1-st byte in the source buffer
    line: 1,    // line 1
    column: 2, // column 2
}
```

Position of `t` in `times`:
```rust
Position {
    offset: 14, // 14-th byte in the source buffer
    line: 2,    // line 2
    column: 6, // column 6
}
```

---