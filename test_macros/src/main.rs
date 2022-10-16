extern crate rand;

use change_base_macro::base;
use list_comprehension_macro::comp;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let mut arr: Vec<u32> = (0..1_000_000).collect();
    arr.shuffle(&mut thread_rng());
    let _new_arr = comp![x + 1 for x in arr if x > 2];


    let _num = base!(1A3C, 16); // = 6716
    // println!("{}", _num);
}