use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;

const DAY: &str = "1"; // TODO: Fill the day
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
        let (mut a_, mut b_): (Vec<_>, Vec<_>) = reader
            .lines()
            .map_while(Result::ok)
            .filter_map(|line| {
                line.split_once("   ").and_then(|(a, b)| {
                    match (a.parse::<usize>(), b.parse::<usize>()) {
                        (Ok(a), Ok(b)) => Some((a, b)),
                        _ => None,
                    }
                })
            })
            .unzip();

        a_.sort();
        b_.sort();

        let answer = a_.into_iter().zip(b_).map(|(a, b)| a.abs_diff(b)).sum();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (a_, b_): (Vec<_>, Vec<_>) = reader
            .lines()
            .map_while(Result::ok)
            .filter_map(|line| {
                line.split_once("   ").and_then(|(a, b)| {
                    match (a.parse::<usize>(), b.parse::<usize>()) {
                        (Ok(a), Ok(b)) => Some((a, b)),
                        _ => None,
                    }
                })
            })
            .unzip();

        let bm_ = b_.into_iter().fold(
            std::collections::HashMap::<usize, usize>::new(),
            |mut m, x| {
                *m.entry(x).or_default() += 1;
                m
            },
        );

        let answer = a_.iter().map(|a| a * bm_.get(a).unwrap_or(&0)).sum();
        Ok(answer)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
