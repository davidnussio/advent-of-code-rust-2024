use adv_code_2024::*;
use anyhow::Result;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc;
use threadpool::ThreadPool;

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"; // TODO: Add the test input

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
}
impl Directions {
    fn vector(&self) -> (isize, isize) {
        match self {
            Directions::Up => (0, -1),
            Directions::Down => (0, 1),
            Directions::Left => (-1, 0),
            Directions::Right => (1, 0),
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Directions::Up => Directions::Right,
            Directions::Down => Directions::Left,
            Directions::Left => Directions::Up,
            Directions::Right => Directions::Down,
        }
    }
}

#[derive(Clone)]
struct Matrix {
    matrix: Vec<Vec<char>>,
    x: usize,
    y: usize,
    direction: Directions,
    visited: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
}

impl Matrix {
    fn new<R: BufRead>(reader: R) -> Self {
        let lines: Vec<String> = reader.lines().flatten().collect();

        let matrix: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

        let visited = HashSet::new();

        let height = matrix.len();
        let width = matrix.first().map(|l| l.len()).unwrap_or(0);

        let x = 0;
        let y = 0;
        let direction = Directions::Up;

        Matrix {
            matrix,
            x,
            y,
            direction,
            visited,
            width,
            height,
        }
    }

    fn init_position(&mut self) {
        let cursor = vec!['^', 'v', '<', '>'];
        if let Some(position) = self
            .matrix
            .iter()
            .flatten()
            .position(|c| cursor.contains(&c))
        {
            self.x = position % self.width;
            self.y = position / self.width;

            self.visited.insert((self.x, self.y));

            let direction: Directions = match self.get() {
                Ok('^') => Directions::Up,
                Ok('v') => Directions::Down,
                Ok('<') => Directions::Left,
                Ok('>') => Directions::Right,
                _ => unreachable!(),
            };
            self.direction = direction;
        }
    }

    fn get(&self) -> Result<&char, &str> {
        self.matrix
            .get(self.y)
            .and_then(|row| row.get(self.x))
            .ok_or("Not found")
    }

    fn walk(&mut self) -> bool {
        self.visited.insert((self.x, self.y));

        let (dx, dy) = self.direction.vector();

        let new_x = self.x as isize + dx;
        let new_y = self.y as isize + dy;

        if new_x < 0 || new_x >= self.width as isize {
            return false;
        }

        if new_y < 0 || new_y >= self.height as isize {
            return false;
        }

        if self.matrix[new_y as usize][new_x as usize] == '#' {
            self.direction = self.direction.turn_right();
            return true;
        }

        self.x = new_x as usize;
        self.y = new_y as usize;

        true
    }

    fn run(&mut self) -> Result<usize> {
        self.init_position();

        loop {
            if !self.walk() {
                break;
            }
        }

        Ok(self.visited.len())
    }

    fn loop_detection(&self) -> bool {
        let mut visited = HashSet::new();
        let mut x = self.x;
        let mut y = self.y;
        let mut direction = self.direction.clone();

        loop {
            // println!();
            // println!("({}, {}, {:?})", x, y, direction);
            // println!(" ({},{})", self.width, self.height);
            // println!("{:?}", visited);
            if !visited.insert((direction.clone(), x, y)) {
                return true;
            }

            let (dx, dy) = direction.vector();
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;

            if new_x < 0
                || new_x >= self.width as isize
                || new_y < 0
                || new_y >= self.height as isize
            {
                return false;
            }

            if self.matrix[new_y as usize][new_x as usize] == '#' {
                direction = direction.turn_right();
                continue;
            }

            x = new_x as usize;
            y = new_y as usize;
        }
    }

    fn run_loop(&mut self) -> Result<usize> {
        let (tx, rx) = mpsc::channel();

        self.init_position();

        let start_x = self.x;
        let start_y = self.y;
        let pool = ThreadPool::new(32); // Adjust the number of threads based on your system

        for y in 0..self.height {
            for x in 0..self.width {
                if start_x == x && start_y == y {
                    continue;
                }
                let mut m = self.clone();
                let tx1 = tx.clone();
                pool.execute(move || {
                    if m.matrix[y][x] == '.' {
                        m.matrix[y][x] = '#';

                        if m.loop_detection() {
                            tx1.send(1).unwrap();
                        }
                        m.matrix[y][x] = '.';
                    }
                });
            }
        }
        pool.join(); // Wait for all tasks to complete
        drop(tx); // Close the channel
        let mut loop_count: i32 = 0;

        for received in rx {
            loop_count += received;
        }

        Ok(loop_count as usize)
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the
        let mut matrix = Matrix::new(reader);
        let answer = matrix.run()?;
        Ok(answer)
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    assert_eq!(5177, result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut matrix = Matrix::new(reader);
        let answer = matrix.run_loop()?;
        Ok(answer)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    assert_eq!(1686, result);
    //endregion

    Ok(())
}
