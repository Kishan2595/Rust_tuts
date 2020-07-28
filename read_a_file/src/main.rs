use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("src/MaraundersMap.txt")
    .expect("Muggles can't open this!");
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)
    .expect("Mischief managed");

    println!("File Contents:\n\n{}", contents);
}
