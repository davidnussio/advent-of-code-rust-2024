use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "00"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"; // TODO: Add the test input

const TEST_RESULT: usize = 142; // TODO: Add the expected result for the test case

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .flatten()
            .map(|line| {
                let number_map: HashMap<&str, &str> = HashMap::from([
                    ("one", "1"),
                    ("two", "2"),
                    ("three", "3"),
                    ("four", "4"),
                    ("five", "5"),
                    ("six", "6"),
                    ("seven", "7"),
                    ("eight", "8"),
                    ("nine", "9"),
                ]);
                let mut result = line.clone();
                for (word, number) in number_map {
                    result = result.replace(word, &number);
                }
                result
            })
            .map(|line| {
                let mut result = 0;
                let first_num = line.find(|c: char| c.is_digit(10));
                let last_num = line.rfind(|c: char| c.is_digit(10));

                if let (Some(first), Some(last)) = (first_num, last_num) {
                    let first_num = line[first..first + 1].parse::<usize>().unwrap() * 10;
                    let last_num = line[last..last + 1].parse::<usize>().unwrap();
                    result = first_num + last_num;
                };

                result
            })
            .sum();

        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(
        TEST_RESULT as usize,
        part1(BufReader::new(TEST.as_bytes()))?
    );

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
