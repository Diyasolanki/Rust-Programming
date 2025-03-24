pub fn run(){
    // vec! is called macros
    // let v = vec![1, 2, 3];

    let mut nums : Vec<i32> = vec![1,2,3,4,5];
    nums.push(6);

    // for x in nums.iter(){
    //     println!("Number : {}" , x);
    // }

    //loop& mutate values 
    //x is a mutable reference (&mut i32 if nums is Vec<i32>).
    for x in nums.iter_mut(){
        // *x dereferences x (getting the actual value).
        *x *= 2;
    }
    println!(" Numbers vector : {:?}" , nums);


}