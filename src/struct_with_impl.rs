
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// Tuple struct
// struct Color(u8 ,u8 ,u8);

struct Person {
    first_name :  String,
    last_name : String
}

impl Person {
    // This is an associated function (similar to a constructor in other languages).
    fn new(first: &str, last: &str) -> Person{
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }
   //get full name
    //In Rust, self is used in struct methods to refer to the current instance of the struct. It allows you to access and modify the struct's fields or call other methods within the struct.
    // Used when a method only reads the struct’s data. Does not take ownership of self.


    // fn full_name(&self) -> String {
    //     format!("{} {}" , self.first_name , self.last_name)
    // }

    // fn change_last_name(&mut self, new_last_name: &str) {
    //     self.last_name = new_last_name.to_string();
    // }

    //name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name , self.last_name)
    }
}



pub fn run(){
    // let c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };

    // let mut p = Person::new("Jhon" , "Doe");
    // println!("{} {}", p.first_name , p.last_name);
    
    // p.first_name = "Mary".to_string();
    // println!("{} {}", p.first_name , p.last_name);

    let p = Person::new("Jhon" , "Doe");
    let tuple_data = p.to_tuple();
    println!("Tuples: {:?}", tuple_data);
    // / println!("{}", p.first_name); // ❌ ERROR: `p` is moved and no longer exists
}
    // println!("Person {}", p.full_name());

    // p.change_last_name("Smith"); 
    // println!("Updated Name: {} {}", p.first_name, p.last_name);



    // access tuple struct
    // let c = Color(255,20,0);
    // println!("Colors: {} {} {}" , c.0  , c.1 , c.2);

    // println!("Colors: {} {} {}" , c.red, c.green, c.blue);
// }