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

fn sequence(n: usize) -> i64 {
    println!("Starting {}", n);
    let mut seq = vec![0, 1];

    while seq.len() <= n {
        let next = find_next_value(&seq);
        seq.push(next);
        println!("{:?}", seq)
    }

    println!("Done {}", seq[n]);
    println!();
    seq[n]
}

fn find_next_value(seq: &Vec<i64>) -> i64 {
    let mut test = *seq.last().unwrap() + 1;

    for (idx, i) in seq.iter().enumerate() {
        for j in &seq[idx + 1..] {
            let diff = j - i;
            let next = j + diff;

            println!("{}, {}, {}", test, next, diff);

            if test == next {
                test = next + 1;
            }
        }
    }
    test
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::sequence;

    #[test]
    fn example_tests() {
        assert_eq!(sequence(0), 0);
        assert_eq!(sequence(1), 1);
        assert_eq!(sequence(2), 3);
        assert_eq!(sequence(3), 4);
        assert_eq!(sequence(4), 9);
        assert_eq!(sequence(1233), 62047);
        // assert_eq!(sequence(6541), 717373);
        // assert_eq!(sequence(7878), 790248);
        // assert_eq!(sequence(1435), 67909);
        // assert_eq!(sequence(6457), 715501);
    }
}
