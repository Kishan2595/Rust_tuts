fn main() {
    
    let n = vec!["Severus", "Dumbeldore", "Harry", "Cedric"];

    for (index, a) in n.iter().enumerate() {
        println!("The name is {} and his ranking is {}", a, index);
    }
}
