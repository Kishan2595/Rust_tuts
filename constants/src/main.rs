const MAXIMUM_NUMBER: u8 = 11;       //const is always mentioned with uppercase and uses snakecase/camelcase
                                     // we also have to specify its data
fn main() {

    for i in (1..MAXIMUM_NUMBER).rev() {
        println!("Launch in T minus {} seconds.", i)
    }
    println!("Launch!")
}
