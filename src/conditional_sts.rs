pub fn run(){
    let number = -2;
   
    if number > 0 {
        println!("{} is positive", number);
    } else if number < 0 {
        println!("{} is negative", number);
    } else {
        println!("{} is equal to 0", number);
    }
}