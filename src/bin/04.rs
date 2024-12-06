use adv_code_2024::*;
use anyhow::Result;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"; // TODO: Add the test input

const TEST2: &str = "\
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let grid: Vec<Vec<char>> = reader
            .lines()
            .flatten()
            .map(|line| line.chars().collect())
            .collect();

        let rows = grid.len();
        let cols = grid[0].len();
        let word = ['X', 'M', 'A', 'S'];
        let mut count = 0;

        // Define directions as (row_offset, col_offset)
        let directions = [
            (0, 1),   // right
            (0, -1),  // left
            (1, 0),   // down
            (-1, 0),  // up
            (1, 1),   // down-right
            (1, -1),  // down-left
            (-1, 1),  // up-right
            (-1, -1), // up-left
        ];

        for r in 0..rows {
            for c in 0..cols {
                for &(dr, dc) in &directions {
                    if matches_word(&grid, r as isize, c as isize, dr, dc, &word) {
                        count += 1;
                    }
                }
            }
        }

        Ok(count)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    // assert_eq!(0, result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let grid: Vec<Vec<char>> = reader
            .lines()
            .flatten()
            .map(|line| line.chars().collect())
            .collect();

        let rows = grid.len();
        let cols = grid[0].len();
        let mut count = 0;

        for r in 1..rows - 1 {
            for c in 1..cols - 1 {
                if matches_word_2(&grid, r as usize, c as usize) {
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    assert_eq!(9, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    assert_eq!(1890, result);
    //endregion

    Ok(())
}

fn matches_word(
    grid: &Vec<Vec<char>>,
    mut r: isize,
    mut c: isize,
    dr: isize,
    dc: isize,
    word: &[char],
) -> bool {
    for &ch in word {
        if r < 0 || r >= grid.len() as isize || c < 0 || c >= grid[0].len() as isize {
            return false;
        }
        if grid[r as usize][c as usize] != ch {
            return false;
        }
        r += dr;
        c += dc;
    }
    true
}

fn matches_word_2(grid: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
    if grid[r][c] != 'A' {
        return false;
    }

    let (rl, rr, ct, cb) = (r - 1, r + 1, c - 1, c + 1);

    let clt = grid[rl][ct];
    let clb = grid[rl][cb];
    let crt = grid[rr][ct];
    let crb = grid[rr][cb];

    if !vec!['M', 'S'].contains(&clt)
        || !vec!['M', 'S'].contains(&crt)
        || !vec!['M', 'S'].contains(&clb)
        || !vec!['M', 'S'].contains(&crb)
    {
        return false;
    }

    if clt == crb || clb == crt {
        return false;
    }

    true
}
