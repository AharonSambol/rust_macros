use list_comprehension_macro::comp;

fn main() {
    let mut i = 0;
    let new_arr = comp![{ i += 1; i } while i < 5];
    println!("{:?}", new_arr); // [1, 2, 3, 4, 5]
}