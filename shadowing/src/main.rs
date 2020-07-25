fn main() {
    
    let x = 10;                  

    {
                                    // this is code blocks 
        let _x = 15;                 // every thing inside the curly braces 
    }                               // are ignored outside.
    println!("x: {}", x);

    let x = "x is a string";        // this is to show that data types change
    println!("{}", x);

    let x = true;
    println!("x is {}", x)

}
