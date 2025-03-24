// Rust doesn't allow us to create a dynamic array because the compiler needs to have a defined size to allocate space for the array.

use std::mem;
pub fn run(){
    // let numbers = [1, 2, 3, 4, 5];
    let nums : [i32;5] = [1,2,3,4,5];
    // println!("{:?}" , nums);

    // arrays are stack allocated 
    println!("Array occupied : {} bytes", mem::size_of_val(&nums));

    //sllice in rust
    // let Slice = &nums[0..2];
    // let Slice: &[i32] = &nums[0..2];
    // println!("{:?}", Slice);

    // iterate over array
    for num in &nums {
        print!(" {} ", num);
    }



}