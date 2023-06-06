# Nano counter 
[![Rust](https://github.com/Acollie/nano_counter/actions/workflows/rust.yml/badge.svg)](https://github.com/Acollie/nano_counter/actions/workflows/rust.yml)

A simple counter implementation of the Python collections counter class, written in Rust.


## Usage
### Importing and using
```rust
use nano_counter::Counter;
let mut counter = Counter::new();
```

### Adding words
```rust

let mut counter = Counter::new();
counter.add(1);
counter.add(2);
counter.add(3);
assert_eq!(counter.count(), 3);
```
### Removing words
```rust
let mut counter = Counter::new();
counter.add(1);
counter.add(2);
counter.add(3);
counter.remove_item(4);
assert_eq!(counter.count(), 2);

```

You can also use a macro to define the counter.
```rust
let mut counter = counter![1, 2, 3];
assert_eq!(counter.count(), 3);
```