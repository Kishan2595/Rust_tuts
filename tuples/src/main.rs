fn main() {

    let tup1 = ("Billie Eilish", "Adele", "Ed Sheeran", ("Chester", "Rob", "Joe"), (13, 5.54, true));    //tuples can hold int, str, float, and can nest tuple itself

    println!("{} my favourite singer ever!", tup1.0);

    println!("{} is lead vocalist of Linkin Park.", (tup1.3).0);

    let (a, b, c, _d, _e) = tup1;                                          // assigning tuples to var

    println!("Listen to Bad Guy by {}.", a);
    println!("Listen to Rolling in the Deep by {}.", b);
    println!("Listen to Perfect by {}.", c);
    //println!("Linkin Park band includes {}", d);                         // still to learn
    //println!("Just to show that {} can also be hold in tuples.", e);
}
