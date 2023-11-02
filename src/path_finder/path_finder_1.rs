//! Task #1: can you reach the exit?
//!
//! You are at position [0, 0] in maze NxN and you can only move in one of the four
//! cardinal directions (i.e. North, East, South, West). Return true if you can reach
//! position [N-1, N-1] or false otherwise.
//!
//! Empty positions are marked `.`.
//! Walls are marked `W`.
//! Start and exit positions are empty in all test cases.
//! Path Finder Series:
//!
//! #1: can you reach the exit?
//! #2: shortest path
//! #3: the Alpinist
//! #4: where are you?
//! #5: there's someone here

#[allow(dead_code)]
fn path_finder(_maze: &str) -> bool {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::path_finder;
    use rstest::rstest;

    #[rstest]
    #[case(
        "\
        .W.\n\
        .W.\n\
        ...\
        ",
        true
    )]
    #[case(
        "\
        ......\n\
        ......\n\
        ......\n\
        ......\n\
        ......\n\
        ......\
        ",
        true
    )]
    #[case(
        "\
        ......\n\
        ......\n\
        ......\n\
        ......\n\
        .....W\n\
        ....W.\
        ",
        false
    )]
    fn basic(#[case] maze: &str, #[case] expect: bool) {
        test_maze(maze, expect);
    }

    fn test_maze(maze: &str, expect: bool) {
        let actual = path_finder(maze);

        assert!(
            actual == expect,
            "Test failed!\n\
             Got:      {}\n\
             Expected: {}\n\
             Maze was: \n\
             {}",
            actual,
            expect,
            maze
        );
    }
}
