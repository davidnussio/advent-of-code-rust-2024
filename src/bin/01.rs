use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (left_lines, right_lines) = parse_data(reader);

        let result = left_lines
            .iter()
            .zip(right_lines.iter())
            .fold(0, |acc, (even, odd)| acc + even.abs_diff(*odd));

        Ok(result)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (left_lines, right_lines) = parse_data(reader);

        let count_map = right_lines.iter().fold(HashMap::new(), |mut acc, right| {
            acc.entry(right).and_modify(|o| *o += 1).or_insert(1);
            acc
        });

        let result = left_lines.iter().fold(0, |acc, left| {
            if let Some(right) = count_map.get(&(left)) {
                return acc + left * right;
            }
            acc
        });

        Ok(result)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn parse_data<R: BufRead>(reader: R) -> (Vec<usize>, Vec<usize>) {
    let (mut left_lines, mut right_lines): (Vec<usize>, Vec<usize>) = reader
        .lines()
        .flatten()
        .fold((Vec::new(), Vec::new()), |(mut left, mut right), line| {
            let mut parts = line.split_whitespace();
            if let (Some(left_num), Some(right_num)) = (parts.next(), parts.next()) {
                left.push(left_num.parse().unwrap());
                right.push(right_num.parse().unwrap());
            }
            (left, right)
        });

    left_lines.sort();
    right_lines.sort();

    (left_lines, right_lines)
}
