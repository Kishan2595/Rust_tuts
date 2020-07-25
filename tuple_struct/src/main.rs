struct WcScores(u8, u8, u8, u8, u8);                    // no need to mention the var, we just need to mention data types

fn main() {

    let virat = WcScores(82, 77, 67, 72, 66);

    println!("Virat Kohli performance in last 5 matches of world cup 2019 were {}, {}, {}, {} and {}.", virat.0, virat.1, virat.2, virat.3, virat.4);
}
