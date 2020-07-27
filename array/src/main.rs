fn main() {
    
    let numbers = [1, 2, 3, 4, 5];       // array created
                                         // numbers [i32, 5] -> by default this happen in background, mentioning is optional. 
    for n in numbers.iter() {
        println!("{}", n);
    }    
    for i in 0..numbers.len() {
        println!("{}", numbers[i]);
    }

    let a = [2; 10];                     // array created, here we are calling 2 for 10 times

    for b in a .iter() {
        println!("{}", b);
    }

}
