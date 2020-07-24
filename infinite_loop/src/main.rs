fn main() {
    
    let mut n = 0;

    loop {
        n += 1;

        if n == 5 {         // the following cmd skips the   
            continue;       // mentioned value from the loop and continues further
        }

        if n == 10 {        // if we do not use break the loop will 
            break;          // continue for infinity
        }
        println!("The value of n is {}", n)
    }
}
