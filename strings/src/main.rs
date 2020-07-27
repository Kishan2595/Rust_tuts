fn main() {
    let mut a_dumbeldore = String::from ("Dumbledore watched her fly away, and as her silvery glow faded he turned back to Snape, and his eyes were full of tears.
After all this time?");

    println!("Length: {}", a_dumbeldore.len());
    println!("String is empty? {}", a_dumbeldore.is_empty());

    for n in a_dumbeldore.split_whitespace() {
        println!("{}", n);
    }

    println!("Does the string contain Dumbeldore? {}", a_dumbeldore.contains("Dumbeldore"));

    a_dumbeldore.push_str("\nAlways!");
    println!("{}", a_dumbeldore);
}
