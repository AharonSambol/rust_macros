# Python fString Macro

Macro for Python-like f-strings in Rust.

Basically just the `format!` macro but lets you put any expression in the braces

## Python:
```python
def add_one(i: int) -> int:
    return i + 1

def main():
    b = 3
    a = f"wow look { add_one(b * 2) }!"
    print(a)
```


## Rust equivalent:
```rust
use py_fstr::f;

fn add_one(i: i32) -> i32 {
    i + 1
}

fn main() {
    let b = 3;
    let a = f!("wow look { add_one(b * 2) }!");
    println!("{a}");
}
```

# Install
Add the following line to your Cargo.toml file:
```toml
py_fstr = "0.1.0"
```
