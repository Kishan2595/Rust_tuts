use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("output.txt")
    .expect("Could not create a file!");

    file.write_all(b"Hello welcome to my page.")         // b is for byte slice
    .expect("Cannot write to file.")
}