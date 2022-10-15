use change_base_macro::base;
use list_comprehension_macro::comp;

fn main() {
    let arr: Vec<u32> = (0..10).collect();
    let new_arr = comp![x.pow(2), x for x in &arr if x % 2 == 0];
    println!("{:?}", new_arr);

    let num = base!(1A3C, 16); // = 6716
    println!("{}", num);
}