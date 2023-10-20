use std::collections::HashMap;

#[allow(dead_code)]
fn grow(array: Vec<i32>) -> i32 {
    array.iter().product()
}

#[allow(dead_code)]
fn solution(word: &str, ending: &str) -> bool {
    if word.len() < ending.len() {
        return false;
    }
    let rword: Vec<char> = word.chars().rev().collect();

    for (i, c) in ending.chars().rev().enumerate() {
        if c != rword[i] {
            return false;
        }
    }
    return true;
}

#[allow(dead_code)]
fn high_and_low(numbers: &str) -> String {
    let nums: Vec<i32> = numbers
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    format!(
        "{} {}",
        nums.iter().max().unwrap(),
        nums.iter().min().unwrap()
    )
}

#[allow(dead_code)]
fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.into_iter().filter(|x| !b.contains(x)).collect()
}

#[allow(dead_code)]
fn duplicate_encode(word: &str) -> String {
    let word = word.to_lowercase();

    let mut set = std::collections::HashSet::new();

    word.chars()
        .map(|c| {
            if word.matches(c).count() > 1 {
                ')'
            } else {
                set.insert(c);
                '('
            }
        })
        .collect::<String>()
}

#[allow(dead_code)]
/// # This implements finding the tribonacci sequence
///
/// The best solution found
/// ```
/// fn tribonacci(signature: &[f64], n: usize) -> Vec<f64> {
///    let mut cache = signature.to_vec();
///
///    cache.resize(n, 0.0);
///
///    for i in 3..cache.len() {
///        cache[i] = cache[i - 1] + cache[i - 2] + cache[i - 3];
///    }
///
///    cache
/// }
/// ```
///
/// # Why its better
/// This approach more elegantly uses the built in utils.
/// Also it has a more readable use of the given variables.
///
/// # What I Think I did better
/// The use of slices is pretty rad.
/// Also I like return.

fn tribonacci(signature: &[f64; 3], n: usize) -> Vec<f64> {
    // your code here
    let mut rv: Vec<f64> = Vec::with_capacity(n);
    for i in signature {
        rv.push(*i);
    }
    for i in 3..(n) {
        rv.push(rv[i - 3..i].iter().sum());
    }
    rv.resize(n, 0.);
    rv
}

#[allow(dead_code)]
/// # This folds a value multiplying all the numbers of a decimal number
///
/// The best solution found
/// ```
/// fn persistence(num: u64) -> u64 {
///    let (_, cnt) = multi(num, 0);
///    cnt
///}
///
///fn multi(num: u64, cnt: u64) -> (u64, u64) {
///    if num < 10 {
///        return (num, cnt)
///    }
///    multi(num.to_string()
///            .chars()
///            .map(|n| n.to_digit(10).unwrap() as u64 )
///            .fold(1, |s, n| s * n), cnt + 1)
///}
/// ```
///
/// # Why its better
/// It has a much better use of iterators and functional methods.
///
/// # What I Think I did better
/// I think mine is faster.
fn persistence(num: u64) -> u64 {
    let mut count = 0;
    let mut hold = num;
    while hold >= 10 {
        let mut temp = 1;
        while hold > 0 {
            temp *= hold % 10;
            hold /= 10;
        }
        hold = temp;
        count += 1;
    }
    count
}

#[allow(dead_code)]
fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    let mut low: usize = ints.len();
    let mut ret: Option<(i8, i8)> = None;

    for (i, item) in ints.iter().enumerate() {
        if i >= low {
            break;
        }
        for (j, jtem) in ints[(i + 1)..low].iter().enumerate() {
            if item + jtem == s {
                ret = Some((*item, *jtem));
                low = j + i + 1;
                break;
            }
        }
    }
    ret
}

#[allow(dead_code)]
fn song_decoder(song: &str) -> String {
    song.split("WUB")
        .map(|s| s.trim())
        .filter(|&s| s != "")
        .collect::<Vec<&str>>()
        .join(" ")
}

#[allow(dead_code)]
fn count_duplicates(text: &str) -> u32 {
    let mut char_count = HashMap::new();

    for c in text.to_lowercase().chars() {
        match char_count.get(&c) {
            Some(&val) => char_count.insert(c, val + 1),
            None => char_count.insert(c, 1),
        };
    }
    char_count.iter().filter(|&val| val.1 > &1).count() as u32
}

#[allow(dead_code)]
fn solution_1(num: i32) -> i32 {
    (3..num).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

/// Failed
#[allow(dead_code)]
fn remove_nb(m: i32) -> Vec<(i32, i32)> {
    let sum = (m * (m + 1)) / 2;
    let mut rv = Vec::new();
    for i in 2..m {
        let temp = sum - i;
        for j in 2..m {
            if temp % j == 0 && (temp - j) / j == i {
                rv.push((i, j));
                break;
            }
        }
    }
    rv
}

#[allow(dead_code)]
fn sum_consecutives(numbers: &Vec<i32>) -> Vec<i32> {
    let mut hold = numbers[0];
    let mut sum = numbers[0];
    let mut result = vec![];
    for i in numbers[1..].iter() {
        if i == &hold {
            sum += i;
        } else {
            result.push(sum);
            hold = *i;
            sum = *i;
        }
    }
    result.push(sum);
    result
}

#[allow(dead_code)]
fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
    use std::collections::BTreeMap;

    let mut factor_sum = BTreeMap::new();

    // n = l.len()
    // m = l.iter().max()
    //
    // total runtime:
    //
    // n * (sqrt(m) + sqrt(m) * log(sqrt(m))) + sqrt(m)
    //
    // O(n * sqrt(m) * log(sqrt(m)))

    // n
    for i in l {
        // sqrt(m)
        let factors = factorize(i);

        // sqrt(m)
        for factor in factors {
            // log(sqrt(m))
            let sum = factor_sum.entry(factor).or_insert(0);
            *sum += i;
        }
    }

    // sqrt(m)
    factor_sum.into_iter().map(|(k, v)| (k, v)).collect()
}

pub fn factorize(n: i64) -> Vec<i64> {
    let mut n = n;
    let mut factor = 2;
    let mut rv = Vec::new();
    while n > 1 {
        if n % factor == 0 {
            rv.push(factor);
            while n % factor == 0 {
                n /= factor;
            }
        }
        factor += 1;
    }
    rv
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basics_sum_of_divided() {
        fn testing(l: Vec<i64>, exp: Vec<(i64, i64)>) -> () {
            assert_eq!(sum_of_divided(l), exp)
        }
        testing(vec![12, 15], vec![(2, 12), (3, 27), (5, 15)]);
        testing(
            vec![15, 21, 24, 30, 45],
            vec![(2, 54), (3, 135), (5, 90), (7, 21)],
        );
    }

    #[test]
    fn test_sample() {
        println!("Sample Tests");

        let input = vec![1, 4, 4, 4, 0, 4, 3, 3, 1];
        let expected = vec![1, 12, 0, 4, 6, 1];
        println!("Input: {:?}", input);
        assert_eq!(sum_consecutives(&input), expected);

        let input = vec![-5, -5, 7, 7, 12, 0];
        let expected = vec![-10, 14, 12, 0];
        println!("Input: {:?}", input);
        assert_eq!(sum_consecutives(&input), expected);
    }

    fn testing(n: i32, exp: Vec<(i32, i32)>) -> () {
        assert_eq!(remove_nb(n), exp)
    }

    #[test]
    fn basics_remove_nb() {
        testing(26, vec![(15, 21), (21, 15)]);
        testing(100, vec![]);
        testing(101, vec![(55, 91), (91, 55)]);
        testing(102, vec![(70, 73), (73, 70)]);
    }

    #[test]
    fn returns_expected_1() {
        assert_eq!(solution_1(10), 23);
        assert_eq!(solution_1(11), 33);
        assert_eq!(solution_1(6), 8);
    }

    #[test]
    fn test_abcde() {
        assert_eq!(count_duplicates("abcde"), 0);
        assert_eq!(count_duplicates("abcdea"), 1);
        assert_eq!(count_duplicates("indivisibility"), 1);
        assert_eq!(count_duplicates("aabcdea"), 1);
        assert_eq!(count_duplicates("aabbdea"), 2);
    }

    #[test]
    fn returns_expected() {
        assert_eq!(song_decoder("WUBAWUBWUBC"), "A C");
        assert_eq!(song_decoder("AWUBWUBWUBBWUBWUBWUBC"), "A B C");
        assert_eq!(song_decoder("WUBAWUBBWUBCWUB"), "A B C");
        assert_eq!(song_decoder("AWUBBWUBC"), "A B C");
    }

    #[test]
    fn sum_pairs_test() {
        let l1 = [1, 4, 8, 7, 3, 15];
        let l2 = [1, -2, 3, 0, -6, 1];
        let l3 = [20, -13, 40];
        let l4 = [1, 2, 3, 4, 1, 0];
        let l5 = [10, 5, 2, 3, 7, 5];
        let l6 = [4, -2, 3, 3, 4];
        let l7 = [0, 2, 0];
        let l8 = [5, 9, 13, -3];
        assert_eq!(sum_pairs(&l1, 8), Some((1, 7)));
        assert_eq!(sum_pairs(&l2, -6), Some((0, -6)));
        assert_eq!(sum_pairs(&l3, -7), None);
        assert_eq!(sum_pairs(&l4, 2), Some((1, 1)));
        assert_eq!(sum_pairs(&l5, 10), Some((3, 7)));
        assert_eq!(sum_pairs(&l6, 8), Some((4, 4)));
        assert_eq!(sum_pairs(&l7, 0), Some((0, 0)));
        assert_eq!(sum_pairs(&l8, 10), Some((13, -3)));
    }

    #[test]
    fn grow_test() {
        assert_eq!(grow(vec![1, 2, 3]), 6);
        assert_eq!(grow(vec![4, 1, 1, 1, 4]), 16);
        assert_eq!(grow(vec![2, 2, 2, 2, 2, 2]), 64);
    }

    #[test]
    fn solution_test() {
        assert_eq!(true, solution("abc", "c"));
        assert_eq!(false, solution("strawberry", "banana"));
    }

    #[test]
    fn high_and_low_test() {
        assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
    }

    #[test]
    fn array_diff_test() {
        assert_eq!(array_diff(vec![1, 2], vec![1]), vec![2]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![1]), vec![2, 2]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![2]), vec![1]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![]), vec![1, 2, 2]);
        assert_eq!(array_diff(vec![], vec![1, 2]), vec![]);
    }

    #[test]
    fn duplicate_encode_tests() {
        assert_eq!(duplicate_encode("din"), "(((");
        assert_eq!(duplicate_encode("recede"), "()()()");
        assert_eq!(duplicate_encode("Success"), ")())())", "should ignore case");
        assert_eq!(duplicate_encode("(( @"), "))((");
    }

    #[test]
    fn persistence_test() {
        assert_eq!(super::persistence(39), 3);
        assert_eq!(super::persistence(4), 0);
        assert_eq!(super::persistence(25), 2);
        assert_eq!(super::persistence(999), 4);
    }
}
