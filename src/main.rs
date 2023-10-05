use codewars::prime_factors::prime_factors;
fn main() {
    let val = 933555431;
    let f = || {
        let x = prime_factors(val);
        println!("{}: {:?}", val, x);
    };
    time_it(f);
    // wait for input
    // std::io::stdin().read_line(&mut String::new()).unwrap();
}

fn time_it<F>(f: F)
where
    F: Fn(),
{
    let t0 = std::time::SystemTime::now();
    f();
    let elapsed = t0.elapsed().unwrap();
    println!("Time elapsed: {}ms", elapsed.as_millis());
}
