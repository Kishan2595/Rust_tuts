struct Colour {
    red: u8,
    green: u8,
    blue:u8
}

fn main() {

    let a = Colour {red:15, green: 45, blue: 255};         // call the struct
    print_colour(&a);                                      // provided value to func through ref
    print_colour(&a);
}

fn print_colour(c: &Colour) {                              // created a func and pass value to it through a ref
    println!("Colours - R: {}, G: {}, B: {}", c.red, c.green, c.blue);
}