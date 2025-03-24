// std::env::args() to retrieve command-line arguments passed to the program when it runs.
use std::env;
pub fn run(){
    let args: Vec<String> = env::args().collect(); // Collect command-line arguments into a vector
    // println!("Args: {:?}" , args);
    let command = args[1].clone();
    // println!("Command : {}" , command);
    let name = "Jadu";
    let city = "Pune";
    if command == "Jadu"{
        println!("Hi {}" , name);
    }
    else if command == "Pune"{
        println!("Hello from {}" , city);
    }
    else{
        println!("Not matching command");
    }

}

