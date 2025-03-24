pub fn run(){
    // let mut hello = String::from("hello");
    // hello.push('w');
    // hello.push_str("orld!");
    // println!("{}" , hello);
    
    // The .capacity() method in Rust is used to check how much memory (in bytes) a String or Vec has allocated without needing to reallocate. This helps optimize performance by avoiding frequent memory allocations.

    let mut s = String::with_capacity(10);
    println!("{}" , s.capacity());
    s.push_str("Hello world");
    println!("After adding a value:  {}" , s.capacity());
    println!("length of s :  {}" , s.len());

    // The assert! macro in Rust is used to check if a condition is true during runtime. If the condition is false, the program panics (crashes).
    assert!(s.capacity() >= 10); //if it is true its do nothing and allocate memory automatically

    // The.trim() method in Rust is used to remove whitespace (spaces, tabs, newlines) from the beginning and end of a string.

    // let t = "   hello world   ".trim();
    // println!("{:?}", t); // Output: "hello world"




}