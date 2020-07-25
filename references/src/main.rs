fn main() {
    
    let mut x = 10;                  // we setup a value for x
    let y = &x;                      // now we refer it to y and z
    let z = &x;                      // which means vakue of x can be accessed  through y and z.

    println!("Value of x is {}", x);
    println!("Value of x is {}", x);
    println!("Value of x is {}\n", x);

    //y += 1;                        // but changes can't be made through ref var as they are immutable.

    //  ** to change the value through ref **

    let mut a = 20;
    let b = &mut a;
    
    *b += 1;                        // to make it mutable we need to add * as shown

    println!("Value of a is {}", b);
}

