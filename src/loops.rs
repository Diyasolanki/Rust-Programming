
pub fn run(){
    // let mut number = 0;
  
    // loop {
    //     number += 1;
    //     print!(" {} ", number);
        
    //     if number >= 5 {
    //         break;
    //     }
    // }

    
    //while loop
    // while number < 6 {
    //     print!(" {} ", number);
        
    //     number += 1;
    // }

    //for loop
    // for i in 1..5 {
    //     print!(" {} ", i);
    // }

    // let x = 2;

    // use of match expression to pattern match against variable x
    // match x {
    //     1 => println!("x is 1"),
    //     2 => println!("x is 2"),
    //     _ => println!("x is something else"),
    // }

    // let numbers = [3; 5];

    // println!("Array of numbers = {:?}", numbers); //it will print 3 5 times

    let mut nums = 0;
    while nums < 10 {
        if nums > 4{
            println!("{}" , "fizz");
        }
        else if nums < 4{
            println!("{}" , "buzz");
        }
        else if nums == 4{
            println!("{}" , "fizzbuzz");
        }
        else{
            println!("{}" , "hey");
        }
        nums += 1;
    }

}