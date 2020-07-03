#[allow(dead_code)]
fn grow(array: Vec<i32>) -> i32 {
    array.iter().fold(1, |acc, x| acc * x)
}

#[allow(dead_code)]
fn solution(word: &str, ending: &str) -> bool {
    if word.len() < ending.len() {
        return false;
    }
    let rword: Vec<char> = word.chars().rev().collect();

    for (i, c) in ending.chars().rev().enumerate() {
        if c != rword[i] { return false; }
    }
    return true;
}

#[allow(dead_code)]
fn high_and_low(numbers: &str) -> String {
    let nums:Vec<i32> = numbers.split(' ').map(|s| {
        s.parse::<i32>().unwrap()
    }).collect();
    format!("{} {}",nums.iter().max().unwrap(), nums.iter().min().unwrap())
}

#[allow(dead_code)]
fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.into_iter().filter(|x| !b.contains(x)).collect()
}

#[allow(dead_code)]
fn duplicate_encode(word:&str)->String {

    let word = word.to_lowercase();

    let mut set = std::collections::HashSet::new();

    word.chars().map(|c| {
        if word.matches(c).count() > 1 {
            ')'
        } else {
            set.insert(c);
            '('
        }
    }).collect::<String>()
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
        rv.push(i.clone());
    }
    for i in 3..(n) {
        rv.push(rv[i-3..i].into_iter().sum());
    }
    rv.resize(n,0.);
    return rv;
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
            println!("{}", temp);
            hold /= 10;
        }
        hold = temp;
        count += 1;
    }
    return count;
}

#[allow(dead_code)]
fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    println!("new {}", s);
    let mut rv: Vec<(&i8, &i8)> = vec![];
    let mut low: usize = ints.len();

    for (i,item) in ints.iter().enumerate() {
        if i >= low { break; }
        for (j, jtem) in ints[(i + 1)..low].iter().enumerate(){
            if item + jtem == s {
                rv.push((item, jtem));
                println!("i:{} j:{}",i ,j + i);
                low = j + i + 1;
                break;
            }
        }
    }
    rv.pop().map(|(i, j)| (*i, *j))
}

#[cfg(test)]
mod tests{
    use super::*;

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
        assert_eq!(array_diff(vec![1,2], vec![1]), vec![2]);
        assert_eq!(array_diff(vec![1,2,2], vec![1]), vec![2,2]);
        assert_eq!(array_diff(vec![1,2,2], vec![2]), vec![1]);
        assert_eq!(array_diff(vec![1,2,2], vec![]), vec![1,2,2]);
        assert_eq!(array_diff(vec![], vec![1,2]), vec![]);
    }

    #[test]
    fn duplicate_encode_tests() {
        assert_eq!(duplicate_encode("din"),"(((");
        assert_eq!(duplicate_encode("recede"),"()()()");
        assert_eq!(duplicate_encode("Success"),")())())","should ignore case");
        assert_eq!(duplicate_encode("(( @"),"))((");
    }

    #[test]
    fn tribonacci_test() {
        assert_eq!(tribonacci(&[0., 1., 1.], 10), vec![0., 1., 1., 2., 4., 7., 13., 24., 44., 81.]);
        assert_eq!(tribonacci(&[1., 0., 0.], 10), vec![1., 0., 0., 1., 1., 2., 4., 7., 13., 24.]);
        assert_eq!(tribonacci(&[0., 0., 0.], 10), vec![0., 0., 0., 0., 0., 0., 0., 0., 0., 0.]);
        assert_eq!(tribonacci(&[1., 2., 3.], 10), vec![1., 2., 3., 6., 11., 20., 37., 68., 125., 230.]);
        assert_eq!(tribonacci(&[3., 2., 1.], 10), vec![3., 2., 1., 6., 9., 16., 31., 56., 103., 190.]);
        assert_eq!(tribonacci(&[1., 1., 1.], 1), vec![1.]);
        assert_eq!(tribonacci(&[300., 200., 100.], 0), vec![]);
        assert_eq!(tribonacci(&[0.5, 0.5, 0.5], 30), vec![0.5, 0.5, 0.5, 1.5, 2.5, 4.5, 8.5, 15.5, 28.5, 52.5, 96.5, 177.5, 326.5, 600.5, 1104.5, 2031.5, 3736.5, 6872.5, 12640.5, 23249.5, 42762.5, 78652.5, 144664.5, 266079.5, 489396.5, 900140.5, 1655616.5, 3045153.5, 5600910.5, 10301680.5]);
    }

    #[test]
    fn persistence_test() {
        assert_eq!(super::persistence(39), 3);
        assert_eq!(super::persistence(4), 0);
        assert_eq!(super::persistence(25), 2);
        assert_eq!(super::persistence(999), 4);
    }
}
