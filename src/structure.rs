pub fn run(){
    struct Person {
        name : String,
        age : u8
    }

    let p = Person {
        name : String::from("Kaily"),
        age : 28
    };
 
    // println!("My name is {} and I am {} years old" , p.name, p.age);
    let Person {name , age} = p;

    println!("Name is {} and age is {}", name, age);
}