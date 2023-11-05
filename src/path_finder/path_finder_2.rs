//! # Task
//!
//! You are at position [0, 0] in maze NxN and you can only move in one of the four cardinal directions (i.e. North, East, South, West).
//! Return the minimal number of steps to exit position [N-1, N-1] if it is possible to reach the exit from the starting position.
//! Otherwise, return None.
//!
//! Empty positions are marked ..
//! Walls are marked W.
//! Start and exit positions are guaranteed to be empty in all test cases.
//!
//! # Path Finder Series:
//!
//! #1: can you reach the exit?
//! #2: shortest path
//! #3: the Alpinist
//! #4: where are you?
//! #5: there's someone here

use std::collections::VecDeque;
use std::fmt::Debug;

#[allow(dead_code)]
pub fn path_finder(maze: &str) -> Option<u64> {
    let mut maze = Maze::new(maze);

    if maze.is_finished(0, 0) {
        return Some(0);
    }

    let mut flood = Flood::new(&mut maze);

    loop {
        match flood.flood_step() {
            FloodStep::Continue => (),
            FloodStep::Failed => return None,
            FloodStep::Finished(distance) => return Some(distance),
        }
    }
}

#[derive(Debug, PartialEq)]
enum FloodStep {
    Continue,
    Failed,
    Finished(u64),
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

        let distance = if let Tile::Seen(d) = self.maze.tiles[y][x] {
            d
        } else {
            return FloodStep::Failed;
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
                return FloodStep::Finished(distance + 1);
            }
            if self.maze.try_set_seen(i, j, distance + 1) {
                self.edges.push_back((i, j));
            }
        }

        FloodStep::Continue
    }
}

#[derive(PartialEq)]
enum Tile {
    Empty,
    Seen(u64),
    Wall,
    Finished,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "[ . ]"),
            Tile::Seen(s) => write!(f, "[{:>3}]]", s),
            Tile::Wall => write!(f, "[ W ]"),
            Tile::Finished => write!(f, "[ F ]"),
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

        tiles[0][0] = Tile::Seen(0);
        tiles[width - 1][height - 1] = Tile::Finished;

        Self {
            tiles,
            width,
            height,
        }
    }

    fn try_set_seen(&mut self, x: usize, y: usize, distance: u64) -> bool {
        if !self.legal_square(x, y) {
            return false;
        }
        self.tiles[y][x] = Tile::Seen(distance);
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

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::path_finder;

    #[test]
    fn fixed_tests() {
        assert_eq!(
            path_finder(".W.\n.W.\n..."),
            Some(4),
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            path_finder(".W.\n.W.\nW.."),
            None,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            path_finder("......\n......\n......\n......\n......\n......"),
            Some(10),
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            path_finder("......\n......\n......\n......\n.....W\n....W."),
            None,
            "\nYour answer (left) is not the expected answer (right)."
        );
    }
}
