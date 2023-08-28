## ref
https://www.flenker.blog/hecto/

## overview
- `document.rs`: file open, column handler
- `editor.rs`: core logic
- `row.rs`: line handler
- `terminal.rs`: wrap tui api

## basics

is ctrl
```rust
fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}
```

cargo clippy

terminal protocol


```rust
// two hack ways to flush, more standardly should directly call `stdout().flush()`
Key::Char(c) => print!("{}\n", c as u8),
Key::Char(c) => println!("{}", c as u8),
```
