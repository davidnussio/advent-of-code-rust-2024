use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
1 1 2 3 4
1 2 3 4 4
1 1 2 3 3
1 2 2 3 3
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let answer = reader.lines().flatten().fold(0, |acc, line| {
            let numbers = get_nums(line);
            let result = if is_safe(&numbers) { 1 } else { 0 };
            acc + result
        });
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i64> {
        let answer = reader.lines().flatten().fold(0, |acc, line| {
            let numbers = get_nums(line);

            let result = if is_safe(&numbers) {
                1
            } else if numbers.iter().enumerate().any(|(i, _)| {
                is_safe(
                    &numbers
                        .iter()
                        .enumerate()
                        .filter_map(|(j, &num)| if i != j { Some(num) } else { None })
                        .collect::<Vec<_>>(),
                )
            }) {
                1
            } else {
                0
            };

            acc + result
        });
        Ok(answer)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn get_nums(line: String) -> Vec<i64> {
    let numbers: Vec<i64> = line
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect();
    numbers
}

fn is_safe(nums: &[i64]) -> bool {
    let diffs: Vec<_> = nums.windows(2).map(|w| w[0] - w[1]).collect();
    diffs.iter().all(|&diff| diff.abs() <= 3 && diff != 0)
        && diffs
            .iter()
            .map(|&diff| diff.signum())
            .collect::<Vec<_>>()
            .windows(2)
            .all(|w| w[0] == w[1])
}
