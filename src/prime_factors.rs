use std::collections::BTreeMap;

pub fn prime_factors(n: i64) -> String {
    let mut n = n as u64;

    let mut factor_map = BTreeMap::new();

    let mut count = 0;
    while n % 2 == 0 {
        n /= 2;
        count += 1;
    }

    if count > 0 {
        factor_map.insert(2, count);
    }

    while n > 1 {
        let p = pollards_rho_pre(n);
        println!("{}: {}", n, p);

        let mut count = 0;
        while n % p == 0 {
            n /= p;
            count += 1;
        }

        factor_map.insert(p, count);
    }

    let mut ret_str = String::new();

    for (p, count) in factor_map {
        let factorization = match count {
            1 => format!("({})", p),
            _ => format!("({}**{})", p, count),
        };
        ret_str.push_str(factorization.as_str());
    }

    ret_str
}

fn pollards_rho_pre(n: u64) -> u64 {
    let mut x = 2;
    loop {
        let p = pollards_rho(n, x);
        if is_prime(p) {
            return p;
        }
        x += 1;
    }
}

fn pollards_rho(n: u64, x: u64) -> u64 {
    let mut x: u64 = x;
    let mut y: u64 = x;
    let mut d: u64 = 1;

    let g = |x: u64| ((x.overflowing_mul(x).0 + 1) % n);

    while d == 1 {
        x = g(x);
        y = g(g(y));

        let z = x.abs_diff(y);
        d = gcd(z, n);
    }

    d
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn is_prime(n: u64) -> bool {
    if n == 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(n: i64, exp: &str) -> () {
        assert_eq!(&prime_factors(n), exp)
    }

    #[test]
    fn basics_prime_factors() {
        testing(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
        testing(17 * 17 * 93 * 677, "(3)(17**2)(31)(677)");
    }
}
