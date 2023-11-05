use codewars::arithmetic_sequence::sequence;
fn main() {
    for i in 0..=5 {
        println!("n: {}", i);
        let seq = sequence(i);
        println!("{:?}", seq);
        println!();
    }
}
