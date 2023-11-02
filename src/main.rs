use codewars::arithmetic_sequence::sequence;
fn main() {
    let ten_to_nine = 10usize.pow(9);
    let seq = sequence(ten_to_nine);
    println!("{:?}", seq);
}
