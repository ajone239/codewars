use codewars::to_postfix::to_postfix;

fn main() {
    let expr = "5+(6-2)*9+3^(7-1)";

    let postfix = to_postfix(&expr);
    println!("{:?}", postfix);
}
