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

use std::collections::VecDeque;
use std::fmt::Debug;

#[allow(dead_code)]
pub fn path_finder(maze: &str) -> bool {
    let mut maze = Maze::new(maze);

    if maze.is_finished(0, 0) {
        return true;
    }

    let mut flood = Flood::new(&mut maze);

    loop {
        match flood.flood_step() {
            FloodStep::Continue => (),
            FloodStep::Failed => return false,
            FloodStep::Finished => return true,
        }
    }
}

#[derive(Debug, PartialEq)]
enum FloodStep {
    Continue,
    Failed,
    Finished,
}

#[derive(Debug)]
struct Flood<'a> {
    maze: &'a mut Maze,
    edges: VecDeque<(usize, usize)>,
}

impl<'a> Flood<'a> {
    fn new(maze: &'a mut Maze) -> Self {
        let mut edges = VecDeque::new();

        edges.push_back((0, 0));

        Self { maze, edges }
    }

    fn flood_step(&mut self) -> FloodStep {
        let (x, y) = match self.edges.pop_front() {
            Some((x, y)) => (x, y),
            None => {
                return FloodStep::Failed;
            }
        };
        // north, east, south, west
        let directions: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let directions = directions
            .iter()
            .filter(|(i, j)| {
                self.maze
                    .legal_square((x as i64 + i) as usize, (y as i64 + j) as usize)
            })
            .map(|(i, j)| ((x as i64 + i) as usize, (y as i64 + j) as usize))
            .collect::<Vec<_>>();

        for (i, j) in directions {
            if self.maze.is_finished(i, j) {
                return FloodStep::Finished;
            }
            if self.maze.try_set_seen(i, j) {
                self.edges.push_back((i, j));
            }
        }

        FloodStep::Continue
    }
}

#[derive(PartialEq)]
enum Tile {
    Empty,
    Seen,
    Wall,
    Finished,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Seen => write!(f, "S"),
            Tile::Wall => write!(f, "W"),
            Tile::Finished => write!(f, "F"),
        }
    }
}

struct Maze {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Maze {
    fn new(maze: &str) -> Self {
        let mut tiles: Vec<Vec<Tile>> = maze
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Tile::Empty,
                        'W' => Tile::Wall,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        let width = tiles[0].len();
        let height = tiles.len();

        tiles[0][0] = Tile::Seen;
        tiles[width - 1][height - 1] = Tile::Finished;

        Self {
            tiles,
            width,
            height,
        }
    }

    fn try_set_seen(&mut self, x: usize, y: usize) -> bool {
        if !self.legal_square(x, y) {
            return false;
        }
        self.tiles[y][x] = Tile::Seen;
        true
    }

    fn legal_square(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        self.tiles[y][x] == Tile::Empty || self.tiles[y][x] == Tile::Finished
    }

    fn is_finished(&self, x: usize, y: usize) -> bool {
        self.tiles[y][x] == Tile::Finished
    }
}

impl Debug for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Maze:")?;
        for row in &self.tiles {
            write!(f, "[")?;
            for tile in row {
                write!(f, "{:?}", tile)?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
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
    #[case(".", true)]
    #[case(MAZE, true)]
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

    const MAZE: &str = "\
.WWW.W....W.W...WWW.W.WW...W.W.........WW..W..W.WW....W...WW...WW..............W...W.........\n\
...W.....WW.....W....W........W....W.W...W..W...W...........W.....WW..........WW.W....W.W...W\n\
.......W....W.WWW.............W..................WW..W...........WWW.WWWW.W.W........W..W....\n\
..W.....W..........W..WW..........W.WWW.....W..WW.W.WW........W....W.....W........W.WW......W\n\
WW....WWW.W..W.W.W.WWW...WW........W..WW..W............WW.....W.....WW..WW....WWWW...W..W...W\n\
...WW..W.W...W.W....W.WW..W.....WW.............W...W......W.....WW......W.W..W........W..W...\n\
...W.W......W.W....WW..W....W.....W.W.W..W..W.W..W.........WW.W....WW.....W....W.W....W..W...\n\
...W.....................W....W.W..W..........WW..W..........W.W..W.......W...W.W...W..W.W..W\n\
..............W...W.W..W.W..W.W..W.W...W.W.WW...W..W......W....W...W...W...W..W....WW.W......\n\
....W.W.....W.WW..W...W......WW.W..W...WW.W.............W.........WWW.W.W...W..W...W.W....W.W\n\
..W..WW..W..W.W.W....WW.W.....W..W............W..........W..W........W..W...WW.WW.W..WWW.....\n\
W...W.W.....W.W.......W..WW.W..W..........W.WWWW.......W......W...W.....W..WWW..WWW.WWWW...WW\n\
..W..W..WWWW...W..W.......W.....W....W...W.W...W...W............W.......WWWW.W.W....WW.....W.\n\
W......WW.W..W.....W.WW........W......WW....W.W.W....WW........WWWW.....W.....WW.W......W..W.\n\
W.....WW......WW..WW......WW..WW.....W...W.WW...WWWW.W.W.......W...W......WW..W...........W.W\n\
.........WW.WW....WW....W.................W.....W..WWW....W...WW.....WW..WW....W......W.W.W.W\n\
................W......W.W............W.W....W...W.W.W.W.W..WW.W...W................W.....W..\n\
W....W.W..W...W......W....W...W.....WW.WW..W...........W...W.W..W..W....W.W...W.W......W.....\n\
.W..W.....W..W.WW....WW......W...WW.W..W.......WWW..W.WW...WWWW.WWW...WW.W...W.WWWW.WW..W...W\n\
..W...W....WW....W.....W.W.......W...WWW..W..W..W...W....W......W.W...W.W.W.W....WW......W..W\n\
.W....WW..W...............W....WWW....W....W....W..W...WW..W....WW......W..............W....W\n\
..WW..WW.....W..W....W.....WW.W.W......W..W....WW...........W..WWW.W...W...W..W.WW.W.W....W..\n\
.......WWW.W.W.W.W.........W..W.....W...W........WW.W.W.WW.W.....W..W.W......WW..WW..WWWW....\n\
.WWWW.......W.WWW..W.W...W.WWW.....W.WWWW.......W..W.W..W.....W.........W.....WW..WWWW..WW.W.\n\
..W.W.W......WW....WWW....W...W.....WW.W..WWW.........W.W..W..WW..W.....W..W.....WW.W.......W\n\
..WW....W.W.W.W...W....WW...WW..........W.....W.WW.W...........W.....W.WW........W...........\n\
..............WW..W.....W.W...W...W....WW.W..WW........W....W..W.WWW..WW..W.........W.W.WW...\n\
WWWW..W......W.....WW.WW........W...W...W..W...WWW..WW..W.W.W.....W....W...W..W.......WW..W..\n\
....W..W........W....W........W....W.WW...W...W.W.WW.......W.W..WWW.....WW.WWWW..W..W.WW.WW.W\n\
.W.....W..W....W.W........WW.....W.........W.....W..W...............W.WW.WW.W........WW.W.W..\n\
W......W........WW.WW.......W........WW.WW.W.....W......WW....W..WW...........WW....W.......W\n\
..W..W..W..W...W....WW...........W.......WW.......W.......WWW.WWW...W.....W....WWWW.WW...W..W\n\
W.WWW.WW..W....W..W.......WW......W..W...WWW....WW...W........WWW....W.WW.W.WWW.....WW..W....\n\
..W......W..W.....WW...W..W.W.W..WWWWW.W......W...W...WWW...W.......W.......WWW.....W..W..W.W\n\
WW........W.....W..W.W...W....W..W.......W.W..W...W..W....W.W...W...W....WW..WW.....W.W.W..W.\n\
W.W.....WW....WWW....WW.W.....W.......W.W..WW.......W...W.W....W...W.W.....W....W....W......W\n\
WWW...W.W..........WW....W..W.WW...W..WW.W..W.W.....W....W....W...WW...WWWW...W...W.WW.W.....\n\
WWW..W.....W..W....W.W..W.WW...W.........W..W........WW.W.W....W...W.........W...WW.WWW.....W\n\
.W..WW.W.WW.W.......W..W....W.W.......WWW...W..........WW....W...W....W.W.W.W..........W.W...\n\
.......WW....W.W...W.....WW.WW.W..W....W.....WW..WW.W.....W.....WWW....W.....W.W...WW..WW....\n\
.........W.W................W.WW.........WWW..WW.......WW.WW.W.W.WW..WWWW.......WW..W.....W..\n\
.WW....WWW.......W...W.....W....W.W..W..................W......W......W....W.WWW....WW.W.....\n\
.W............W.............W.W...WW....W.........W.W.W.........W..W......W..W.W.W.....W....W\n\
.W.........W............WW..WWW....W.....W....W..W.........W....W...WW...WW...W...W..W......W\n\
.W.........W..W.........W...W.....WW..W...W...WWWW..WW....W.......WW.W.WW....W......W..WWW...\n\
..W...WW.W...W.WW.....WW..W......W.WW.........W....W..W.....WWWW..WW....W..W......W..........\n\
.W..W..W........W.....W...WWW..W.....W....W....WW...W.WW...W......WWW..W........W...W....W.WW\n\
.....W....W....WW...........W..W........W......W...W....WW...............W....W....WW..W....W\n\
W.......WW..WW......W.......W..............W....W..WW.....WWWWWW...W.W.W..WW..W.......W......\n\
.W.W...W.........WW...W..WWW.....W.W......W.W..W......WW..WW..W.W.W..W.............W.....W...\n\
WW.........W...W....W....W......W.W.....W......WWW...W..W...........WWW..W.W.W..W.....WWWW...\n\
...W.W..W..W.WW....W........W..W..W.W.W......W....W.W...WW..W....W.W........WW.....W..W.....W\n\
.WW..WW....W.W....W.WWW...W..W........W..W.....W..WW.....WW.WW.W..W......W...W....W.W...W.WW.\n\
.....W..........WW......W..WW..W....WW...WWW...W.W.W..W....W....W...W......W..W.W.....W......\n\
........W.W...W.WW...W.W......WW.........W..W..W......W..W.W......W........W.....WW.....W..W.\n\
W....W.....W....W.WWWW.W..W.W..W...WW..WW.........WWW......W.......W.W.W......WW....W....W...\n\
W.W......W...W...WW..W.WW.....W.W.WWWW.W.WWWW...W.W...W..WW...WW..W.........WW..WW.W..W....WW\n\
WWW.WW...W.W.......W........W...W.WWW.W.WW.W.W...WW...W..W...........WW..W.....W...W.....W...\n\
.W......W........W...W....W.WW.....W..W..W.......W...WW.....W....WWWW.W.......W......WW.WWW..\n\
W..WWW...W.W....WW..W.WWW..WWW......W.W.W..WWW..W.....W..W.W.W......W....WW...WWW.......WW..W\n\
..W.W...WW.W.WW....W.....W..W....W..........W.W...W.WW...W........W........W....WW......W..W.\n\
.WW..W....W.WW...........WWW.W....W.....W..W..W..W..W.WW.W.WWWW.....WWWWW.......W.....W.W..W.\n\
.WW.......WW.W..W..W..WW......W......W.WW.W.W...W.......W...WW..........W..WW....W.....WW.W..\n\
.WW...WW.WWWWW......W.W......W.....WW.W....WW...W...........W...W..W...W...W.W..W..WW.....W.W\n\
W..W....WWW...WW....WWW.........W.W.W..WW..WW....W..W...WW...W.WW...WW....WW..............W.W\n\
.WW....W..WW.W..W......W..W........WW...W..W.WW.....W..WW.W..WWW.WWW...W........W..W.W.W....W\n\
W.WWWW..W.W...........W.W.....W..W..WW...W...W..W.WWW........W....WW...W.W.....WW..W.W....W..\n\
WW.WW......WW.W...W.W..W..W.W.WWWW.....WW..W....W.....W...W..........W..W....WWWW...W......W.\n\
..W..W.....W..W.W..WW.WWW...WW...W..............W..WWWW.........WW.W.W......WWW............W.\n\
...........WW.....WW.WW......W..W......WW........W...WW.WWWWW...W.WW...W........W.WW.W..W.WW.\n\
..WW........WWW......W....W.W....WW..WW....W.W...W.W...W..WW....W.....WW.WWW.....W..W....W...\n\
..........W...W.W..W.WWW...W...WWW.W.W.W..W...WW..WW.....W.......WWW...W....WW..............W\n\
..............W..WWW.....W.W..W.....W......W.W..W.WW....W.W.WW.........WW.W....W...W.W.......\n\
W.W...W...W........WW.W.W.WW.WWWWW.W..W..WW....W...W.....WW............W......W..............\n\
...W....WW...W...W..W..........W.W....WW..W...WW.W.W...W....W.W....WW.WW...WW...W..WWW.....W.\n\
W...W..W.W........W......W.....W..............WW.W.W..W..WW....W..W.WW.W.W.WW....W.W...W....W\n\
...W.W.....W.WW..W....W....W.WW..W..W......W....WW.W...W...WW....W.WWW.W...WW.W....WW..W.W...\n\
.W.W.W..W...W..W......W.WWW....W..W......W.......W.W.WWW...WW...WWW..W...W........W.W...WWW..\n\
..WW....WW..W..WWWWWW.W.W......W.....W.......W..W...WWW....WW............W..W..WW..W....WW..W\n\
WWWWWW..WWW..W...WW..WW.WW....W...W..WW........WW.....W........WW.W.W..W..WW..W...W..W.W.....\n\
WW.WW.W..W.W.W.W........WW..W........W.....W.W.W...W.WW.W..W........W..W.W.WW..W.............\n\
W.W...WW.WWW.W.WW.WW.W..WW.W............W....W.....W...WW.W....WWWW.........WWW..W..W....W...\n\
.W.WW..W.W..W.W..WW...........W....W.W.WW.WW..W.W..W..W.W....W...........W.WWW.W.WW.WW..W....\n\
.WWWWW........WW....W......WW....W.W.WW..W.W.W....WW......WW.....W.WWWW..WW.W.W.W.W.....W....\n\
........W....W.W.....W..W.....W..W.W.W....W.W..WW..W.W...W..W.....W...WW...W............WW...\n\
.W..W.WW..W..WW....W.......WWWW..W.........W....W.....W.WW.......W...W.W.W...WW...W....W.....\n\
.W..W...W..........W......W.......WW........W....W....W...W..W...W............WWW.WW....WWW.W\n\
.W....W.W.W.....W.W..W.WWW.W.WWW.........WW..............W.WW.WW...W...W........W.W.....W.W..\n\
.W.....W......W...W........W....WWW.W..W.W.W...W..W...W.....W....W........W.WWW....W...W...W.\n\
W.....WW....W.W.WW.W..WWW....W.......W...W.....W..WWW....WW..W...W...W........WW....W.W......\n\
..W..W....W......W..........WW.WW.....W.W.......W.....W.WWW..W.W.........W.W....WW.......W...\n\
.W.WW...W....W.W.W.W.W.....W....W..W.W......W.W..W.W...WWW..W......W....WW.W.W.W..W.....WWW..\n\
W.W.W...W.......W...W.W.W..W...W..W........W.....W..W.....W.......W.WW..WW..W.W.WW...W.....W.\
";
}
