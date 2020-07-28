struct Rectangle {
    width: u32,
    length: u32
}
   
impl Rectangle {

    fn print_description(&self) {
        println!("Rectangle {} * {}", self.width, self.length)
    }

    fn is_square(&self) -> bool {
        self.width == self.length
    }
}

fn main() {

    let my_rect = Rectangle {width: 10, length: 15};

    my_rect.print_description();
    
    println!("Rectangle mentioned here is a square: {}", my_rect.is_square());
}
