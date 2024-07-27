pub fn minimum_cost(
    source: String,
    target: String,
    original: Vec<char>,
    changed: Vec<char>,
    cost: Vec<i32>,
) -> i64 {
    let mut graph = vec![vec![0; 26]; 26];

    for i in 0..original.len() {
        let from = original[i] as usize - 'a' as usize;
        let to = changed[i] as usize - 'a' as usize;
        let cost = cost[i];

        if graph[from][to] > cost || graph[from][to] == 0 {
            graph[from][to] = cost;
        }
    }

    for k in 0..26 {
        for i in 0..26 {
            for j in 0..26 {
                if graph[i][k] == 0 || graph[k][j] == 0 {
                    continue;
                }

                let new_cost = graph[i][k] + graph[k][j];

                if graph[i][j] > new_cost || graph[i][j] == 0 {
                    graph[i][j] = new_cost;
                }
            }
        }
    }

    let siter = source.chars();
    let titer = target.chars();

    let ziter = siter.zip(titer);

    let mut cost = 0;
    for (s, t) in ziter {
        if s == t {
            continue;
        }

        let from = s as usize - 'a' as usize;
        let to = t as usize - 'a' as usize;

        let change_cost = graph[from][to] as i64;

        if change_cost == 0 {
            cost = -1;
            break;
        }

        cost += change_cost;
    }

    cost
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "abcd",
        "acbe",
        vec!['a','b','c','c','e','d'],
        vec!['b','c','b','e','b','e'],
        vec![2,5,5,1,2,20],
        28,
    )]
    #[case(
        "aaaa",
        "bbbb",
        vec!['a','c'],
        vec!['c','b'],
        vec![1,2],
        12,
    )]
    #[case(
        "aaaa",
        "bbbb",
        vec!['a','c', 'a'],
        vec!['c','b', 'c'],
        vec![3 ,2, 2],
        16,
    )]
    #[case(
        "abcd",
        "abce",
        vec!['a'],
        vec!['e'],
        vec![10000],
        -1,
    )]
    fn test_func(
        #[case] source: String,
        #[case] target: String,
        #[case] original: Vec<char>,
        #[case] changed: Vec<char>,
        #[case] cost: Vec<i32>,
        #[case] answer: i64,
    ) {
        assert_eq!(
            minimum_cost(source, target, original, changed, cost),
            answer
        );
    }
}
