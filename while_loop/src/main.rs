fn main() {
    let mut n = 1;
    
    while n <= 50 {

        n += 1;

        if n % 5 == 0 {               // if n is multiple of 5
            println!("n is {}", n);
        }        
    }
}
