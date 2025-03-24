pub fn run(){
    // let arr1 = [1,2,3];
    // let arr2 = arr1;

    // vertors are non-primitive in rust mean we can't add one array value in another 
    // so we have to pase refrence of that array so we can get using pointer
    
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("{:?}" , (&vec1 , vec2));
}