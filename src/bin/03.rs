use adv_code_2024::*;
use anyhow::Result;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;

const DAY: &str = "03"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

// xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
const TEST: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"; // TODO: Add the test input

// 2*4 + 5*5 + 11*8 + 8*5

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        let res = reader.lines().flatten().fold(0, |acc, line| {
            re.captures_iter(&line).fold(acc, |acc, cap| {
                acc + cap[1].parse::<usize>().unwrap_or(0) * cap[2].parse::<usize>().unwrap_or(0)
            })
        });

        Ok(res)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    assert_eq!(168539636, result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let re_line = Regex::new(r"don\'t\(\)(.*?)do\(\)").unwrap();
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        let line = reader.lines().flatten().join("");
        let line = re_line.replace_all(&line, "");
        let res = re.captures_iter(&line).fold(0, |acc, cap| {
            acc + cap[1].parse::<usize>().unwrap_or(0) * cap[2].parse::<usize>().unwrap_or(0)
        });

        Ok(res)
    }

    // 110226078 to low
    assert_eq!(48, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    assert_eq!(97529391, result);
    //endregion

    Ok(())
}
