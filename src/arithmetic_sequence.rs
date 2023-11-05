//! Consider a sequence, which is formed by the following rule: next term is taken as the smallest possible non-negative integer, which is not yet in the sequence, so that no 3 terms of sequence form an arithmetic progression.
//!
//! Example
//!
//! f(0) = 0 -- smallest non-negative
//! f(1) = 1 -- smallest non-negative, which is not yet in the sequence
//! f(2) = 3 -- since 0, 1, 2 form an arithmetic progression
//! f(3) = 4 -- neither of 0, 1, 4, 0, 3, 4, 1, 3, 4 form an arithmetic progression, so we can take smallest non-negative, which is larger than 3
//! f(4) = 9 --  5, 6, 7, 8 are not good, since 1, 3, 5, 0, 3, 6, 1, 4, 7, 0, 4, 8 are all valid arithmetic progressions.
//!
//! etc...
//!
//! The task
//!
//! Write a function f(n), which returns the n-th member of sequence.
//!
//! Limitations
//!
//! There are 1000 random tests with 0 <= n <= 10^9, so you should consider algorithmic complexity of your solution.

use std::collections::HashSet;

pub fn sequence(n: usize) -> i64 {
    let mut n = n;
    let mut rv = 0;

    let mut sum_of_three = 1;
    let mut power_of_three = 1;

    while n > 0 {
        let odd = n & 1;
        let count = (n / 2) + odd;

        rv += count * sum_of_three;

        sum_of_three += power_of_three;
        power_of_three *= 3;
        n >>= 1;
    }

    rv as i64
}

pub fn sequence_less_slow(n: usize) -> i64 {
    if n == 0 {
        return 0;
    }

    let mut sum_of_three: Vec<i64> = vec![0, 1];
    let mut power_of_three = 1;

    let mut rv = 0;
    for i in 1..=n {
        let log = count_zeros_until_one(i) + 1;

        if log > sum_of_three.len() - 1 {
            sum_of_three.push(sum_of_three[log - 1] + power_of_three);
            power_of_three *= 3;
        }
        rv += sum_of_three[log];
    }

    rv
}

fn count_zeros_until_one(n: usize) -> usize {
    let mut n = n;
    let mut log = 0;

    while n & 1 == 0 {
        n >>= 1;
        log += 1;
    }
    log
}

pub fn sequence_slow(n: usize) -> i64 {
    let mut seq = Sequence::new(vec![0, 1], n);

    loop {
        match seq.get(n) {
            Some(i) => {
                return i;
            }
            None => {
                seq.find_next_value();
            }
        }
    }
}

#[derive(Debug)]
struct Sequence {
    seq: Vec<i64>,
    bad_vals: HashSet<i64>,
}

impl Sequence {
    fn new(seq: Vec<i64>, n: usize) -> Self {
        Self {
            seq,
            bad_vals: HashSet::with_capacity(n),
        }
    }

    fn get(&self, idx: usize) -> Option<i64> {
        self.seq.get(idx).copied()
    }

    fn find_next_value(&mut self) {
        let last = *self.seq.last().unwrap();

        for i in self.seq.iter() {
            let bad_val = last + last - i;
            if !self.bad_vals.contains(&bad_val) {
                self.bad_vals.insert(bad_val);
            }
        }

        let mut next = last + 1;

        while self.bad_vals.contains(&next) {
            next += 1;
        }

        self.seq.push(next);
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::sequence;
    use rstest::rstest;

    #[rstest]
    #[case(0, 0)]
    #[case(1, 1)]
    #[case(2, 3)]
    #[case(3, 4)]
    #[case(4, 9)]
    #[case(5, 10)]
    #[case(1233, 62047)]
    #[case(1435, 67909)]
    #[case(6457, 715501)]
    #[case(6541, 717373)]
    #[case(7878, 790248)]
    fn example_tests(#[case] n: usize, #[case] expected: i64) {
        assert_eq!(sequence(n), expected);
    }
}
