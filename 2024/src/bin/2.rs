use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;

const DAY: &str = "2";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn check_report_orig(record: &[i32]) -> bool {
        let is_desc = record[0] > record[1];

        for i in 0..(record.len() - 1) {
            let a = record[i];
            let b = record[i + 1];
            if is_desc && a < b || !is_desc && a > b {
                return false;
            }

            let diff = (a - b).abs();
            if !(1..=3).contains(&diff) {
                return false;
            }
        }
        true
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .map_while(Result::ok)
            .map(|line| {
                let report = line.split(" ").flat_map(|s| s.parse::<i32>()).collect_vec();
                check_report_orig(&report) as usize
            })
            .sum::<usize>();
        Ok(answer)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .map_while(Result::ok)
            .map(|line| {
                let report = line.split(" ").flat_map(|s| s.parse::<i32>()).collect_vec();

                if check_report_orig(&report) {
                    return 1;
                }
                for i in 0..report.len() {
                    let mut changed_report = report.clone();
                    changed_report.remove(i);
                    // println!("{}: {:?}", i, changed_report);
                    if check_report_orig(&changed_report) {
                        return 1;
                    }
                }
                0
            })
            .sum::<usize>();
        Ok(answer)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
