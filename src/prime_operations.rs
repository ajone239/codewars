use std::collections::HashMap;

#[allow(dead_code)]
pub fn prime_operations(x: u64, y: u64) -> u32 {
    if x == y {
        return 0;
    }

    println!("{} {}", x, y);

    let xfs = factorize(x);
    let yfs = factorize(y);

    println!("{:?}", xfs);
    println!("{:?}", yfs);

    let mut hs = HashMap::new();

    merge(&mut hs, xfs);
    merge(&mut hs, yfs);

    println!("{:?}", hs);

    hs.iter().filter(|(_, v)| v >= &&1).map(|(_, v)| v).sum()
}

fn merge(ops_count: &mut HashMap<u64, u32>, factors: HashMap<u64, u32>) {
    for (f, c) in factors {
        let count = ops_count.entry(f).or_insert(0);
        let c = c as i32;
        let sign_count = *count as i32;

        *count = (sign_count - c).unsigned_abs();
    }
}

fn factorize(n: u64) -> HashMap<u64, u32> {
    let mut n = n;
    let root_n = (n as f64).sqrt().ceil() as u64;
    let mut rv = HashMap::new();
    for factor in 2..root_n {
        if n % factor != 0 {
            continue;
        }
        let mut count = 0;
        while n % factor == 0 {
            n /= factor;
            count += 1;
        }
        rv.insert(factor, count);
    }
    if n > 1 {
        rv.insert(n, 1);
    }

    rv
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::prime_operations;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(x: u64, y: u64, expected: u32) {
        assert_eq!(
            prime_operations(x, y),
            expected,
            "{ERR_MSG} with x = {x}, y = {y}"
        )
    }

    #[test]
    fn test_equal() {
        dotest(1, 1, 0);
        dotest(123456789, 123456789, 0);
    }

    #[test]
    fn test_easy() {
        dotest(2, 3, 2);
        dotest(5156486548, 5, 5);
    }

    #[test]
    fn test_hard1() {
        dotest(206158430209, 9664475137, 3);
    }
    #[test]
    fn test_hard2() {
        dotest(51539607551, 824633720831, 2);
    }
    #[test]
    fn test_hard3() {
        dotest(1000000000000, 333333333333, 32);
    }

    #[test]
    fn test_hard4() {
        dotest(549755813888, 847288609443, 64);
    }
}
