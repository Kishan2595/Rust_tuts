fn main() {
    let mut my_vector = vec![1, 2, 3, 4];

    my_vector.push(49);
    my_vector.remove(1);           // within parantheses we mentioned the index value

    for n in my_vector.iter() {
        println!("{}", n);
    }
}
