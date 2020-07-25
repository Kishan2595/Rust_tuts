fn main() {
    
    check_numbers(10);             // calling the func
}

fn is_even(num: u32) -> bool {     // create a func by assigning a var, param, type and then arg.
    num % 2 == 0                   // retun can also be used here so don't panic if you see it in some code.
}

fn check_numbers(num: u32) {       // creating another func
    for n in 1..num {
        if is_even(n) {
            println!("{} is even.", n);
        }
        else {
            println!("{} is odd.", n);
        }
    }
}