# Control Flow in Rust

## Conditionals

### Basic Operators
- Rust uses standard conditional operators (`==`, `!=`, `<`, `>`, `<=`, `>=`).

### `if` Statements
- Follows the typical `if`, `else if`, `else` hierarchy.
- Braces `{}` are required for blocks, even for single statements.

```rust
if condition {
    // code
} else if other_condition {
    // code
} else {
    // code
}
```

### `match` Statements
- Similar to switch/case in other languages.
- Pattern matching on values.

```rust
match value {
    pattern1 => { /* code */ },
    pattern2 => { /* code */ },
    _ => { /* default */ },
}
```

---

## Loops

### `for` Loops
- Iterate over ranges or collections.

```rust
for i in start..stop {
    // start is inclusive, stop is exclusive
}
for i in start..=stop {
    // inclusive at both ends
}
for i in (start..stop).step_by(n) {
    // custom step
}
for item in iterable {
    // iterate over items
}
```

### `while` Loops

```rust
while condition {
    // code
}
```

### Loop Control

- `break` exits the current loop.
- `continue` skips to the next iteration.
- You can use labels to break out of outer loops:

```rust
'label: loop {
    // ...
    break 'label;
}
```

---

## Labels

- Any loop can be labeled for more granular control with `break` or `continue`.

```rust
'outer: for i in 0..10 {
    for j in 0..10 {
        if some_condition {
            break 'outer;
        }
    }
}
```