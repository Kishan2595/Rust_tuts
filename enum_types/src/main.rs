enum Houses {         // basically we are creating types
    Gryffindor,
    Slytherin,
    Ravenclaw,
    Hufflepup
}
fn main() {

    let harry_house:Houses = Houses::Gryffindor;   // here we define a type for variable

    match harry_house {                            // now for demonstration we use match func. to compare the type
        Houses::Gryffindor => println!("Harry chose his house to be Gryffindor."), 
        Houses::Slytherin => println!("This is house Slytherin"),
        Houses::Ravenclaw => println!("This is house Ravenclaw"),
        Houses::Hufflepup => println!("This is house Hufflepup"),
    }
}
