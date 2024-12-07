use adv_code_2024::*;
use anyhow::Result;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"; // TODO: Add the test input

fn get_input<R: BufRead>(reader: R) -> Result<Vec<(usize, Vec<usize>)>> {
    let r = reader
        .lines()
        .flatten()
        .map(|line| {
            let mut parts = line.split(": ");
            let result = parts.next().unwrap().parse::<usize>().unwrap();
            let values = parts
                .next()
                .map(|r| {
                    r.split_whitespace()
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            (result, values)
        })
        .collect::<Vec<_>>();
    Ok(r)
}

fn search(numbers: &[usize], target: usize, index: usize, current_value: usize) -> bool {
    if index == numbers.len() {
        return current_value == target;
    }

    let next_num = numbers[index];

    let add_val = current_value + next_num;
    if add_val <= target {
        if search(numbers, target, index + 1, add_val) {
            return true;
        }
    }

    let mul_val = current_value * next_num;
    if mul_val <= target {
        if search(numbers, target, index + 1, mul_val) {
            return true;
        }
    }

    false
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let lines = get_input(reader)?;
        let answer = lines
            .iter()
            .map(|(result, numbers)| {
                let res = result;
                let found = search(&numbers, *result, 1, numbers[0]);

                if found {
                    return res;
                } else {
                }
                &0
            })
            .sum::<usize>();
        println!("Trovate: {}", answer);
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    assert_eq!(21572148763543, result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(_reader: R) -> Result<usize> {
        Ok(0)
    }

    assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    assert_eq!(0, result);
    //endregion

    Ok(())
}
