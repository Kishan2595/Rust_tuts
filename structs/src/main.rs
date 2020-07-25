struct Colour {
    red: u8,
    green: u8,
    blue: u8
}

fn main() {

    let bg = Colour {red: 255, green: 75, blue: 15};

    println!("values are {}, {}, {}.", bg.red, bg.green, bg.blue);

    // here in above case we can't mutate so we do the following changes..

    let mut bg_1 = Colour {red: 255, green: 75, blue: 15};
    
    bg_1.blue = 45;

    println!("values are {}, {}, {}.", bg_1.red, bg_1.green, bg_1.blue);
}
