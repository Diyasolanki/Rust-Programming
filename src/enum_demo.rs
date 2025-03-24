// Enums (or enumerations) is a user-defined data type that allows us to select a value from a list of related values.

pub fn run() {
    // enum Direction {
    //     North,
    //     East,
    //     South,
    //     West,
    // }

    
    // let north = Direction::North;
    // let east = Direction::East;
    // let south = Direction::South;
    // let west = Direction::West;

    
    // println!("{:?}", north);
    // println!("{:?}", east);
    // println!("{:?}", south);
    // println!("{:?}", west);

    #[derive(Debug)]
    enum MyResult {
        Score(f64),
        Valid(bool),
    }

    let num = MyResult::Score(9.2);
    let is_valid = MyResult::Valid(true);
    println!(" Number : {:?}", num);
    println!("Is Valid : {:?}" ,is_valid);
}