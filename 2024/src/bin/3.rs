use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;

const DAY: &str = "3";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5)";
const TEST2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("regex compilation");
        let answer = reader
            .lines()
            .flatten()
            .map(|line| {
                re.captures_iter(&line)
                    .filter_map(|caps| {
                        let (_, [a, b]) = caps.extract();
                        match (a.parse::<usize>(), b.parse::<usize>()) {
                            (Ok(a), Ok(b)) => Some(a * b),
                            _ => None,
                        }
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();
        Ok(answer)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let re = Regex::new(r"(?<do>do\(\))|(?<dont>don't\(\))|mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)")
            .expect("regex compilation");

        let mut enabled = true;
        let answer = reader
            .lines()
            .flatten()
            .map(|line| {
                re.captures_iter(&line)
                    .filter_map(|caps| {
                        if caps.name("do").is_some() {
                            enabled = true;
                            None
                        } else if caps.name("dont").is_some() {
                            enabled = false;
                            None
                        } else if let (Some(a), Some(b)) = (caps.name("a"), caps.name("b")) {
                            match (a.as_str().parse::<usize>(), b.as_str().parse::<usize>()) {
                                (Ok(a), Ok(b)) if enabled => Some(a * b),
                                _ => None,
                            }
                        } else {
                            None
                        }
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();
        Ok(answer)
    }

    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}
