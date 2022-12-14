# Python Comprehension Macro

Macro for Python-like list\dict comprehensions in Rust.

## Python:
```python
even_squares = [x**2 for x in arr if x % 2 == 0]
flatten_matrix = [x for row in arr for x in row]
dict_comp = {x: len(x) for x in arr}
```

## Rust equivalent:
```rust
let even_squares = comp![x.pow(2) for x in arr if x % 2 == 0]
let flatten_matrix = comp![x for row in arr for x in row]
let dict_comp = comp!{x: x.len() for x in arr}
```

# Install
Add the following line to your Cargo.toml file:
```toml
list_comprehension_macro = "0.1.1"
```

# Examples
```rust
use list_comprehension_macro::comp;

fn main() {
    let arr: Vec<u32> = (0..10).collect();
    let new_arr = comp![x.pow(2) for x in arr if x % 2 == 0];
    println!("{:?}", new_arr); // [0, 4, 16, 36, 64]
}
```

```rust
use list_comprehension_macro::comp;

fn main() {
    let arr: Vec<Vec<u32>> = vec![vec![1, 2], vec![3, 4]];
    let new_arr = comp![x for row in arr for x in row];
    println!("{:?}", new_arr); // [1, 2, 3, 4]
}
```

Multiple nesting allowed:

```rust
use list_comprehension_macro::comp;

fn main() {
    let arr: Vec<Vec<u32>> = vec![vec![1, 2], vec![3, 4, 5], vec![6]];
    let new_arr = comp![comp![x + 10 for x in row] for row in arr if row.len() > 1];
    println!("{:?}", new_arr); // [[11, 12], [13, 14, 15]]
}
```

HashMap comprehension (curly brackets for readability):

```rust
use list_comprehension_macro::comp;

fn main() {
    let arr: Vec<&str> = vec!["hello", "World!"];
    let new_arr = comp!{x: x.to_uppercase() for x in arr};
    println!("{:?}", new_arr); // {"hello": "HELLO", "World!": "WORLD!"}
}
```

Even `while` loops are allowed:

```rust
use list_comprehension_macro::comp;

fn main() {
    let mut i = 0;
    let new_arr = comp![{ i += 1; i } while i < 5];
    println!("{:?}", new_arr); // [1, 2, 3, 4, 5]
}
```

# Iterators

Instead of calculating the whole comprehension at once, you might want to get a lazy iterator. To do that use `i_comp!` instead of `comp!`. However there are a few limitations:

* Only one loop is allowed (and only a `for` loop)
* You must pass it an iterator

## Example:
```rust
use list_comprehension_macro::i_comp;

fn main() {
    let arr: Vec<u32> = vec![1, 2, 3, 4];
    let mut iter = i_comp![x * 2 for x in arr.iter() if *x != 3];
    println!("{}", iter.next().unwrap()); // 2
    println!("{}", iter.next().unwrap()); // 4
    println!("{}", iter.next().unwrap()); // 8
}
```




# Note

A comprehension like this:
```rust
comp![x for x in arr]
```
Will move `arr` which means you won't be able to use it after the comprehension. To prevent moving the value you can pass a pointer instead:
```rust
comp![*x for x in &arr]
```