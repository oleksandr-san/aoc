use std::result::Result::Ok;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

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

    fn check_report(report: &[i32], mut tolerations: usize) -> bool {
        let (mut asc, mut start_idx) = if report[0] < report[1] {
            (true, 1)
        } else if report[0] > report[1] {
            (false, 1)
        } else {
            (report[1] < report[2], 2)
        };
        let mut prev = report[start_idx - 1];

        for v in &report[start_idx..] {
            if v.abs_diff(prev) < 1 || v.abs_diff(prev) > 3 {
                if tolerations != 0 {
                    tolerations -= 1;
                    continue;
                } else {
                    println!("difference condition is not met for {:?}: {} -> {}", report, prev, v);
                    return false;
                }
            }

            if (v > &prev) != asc {
                if tolerations != 0 {
                    tolerations -= 1;
                    // asc = !asc;
                    continue;
                } else {
                    println!("order condition is not met for {:?}: {} -> {}", report, prev, v);
                    return false;
                }
            }

            prev = *v;
        }

        true
    }

    fn check_report_orig(record: &[i32]) -> bool {
        let is_desc = record[0] > record[1];

        for i in 0..(record.len() - 1) {
            let a = record[i];
            let b = record[i + 1];
            if is_desc && a < b || !is_desc && a > b {
                return false;
            }

            let diff = (a - b).abs();
            if diff < 1 || diff > 3 {
                return false;
            }
        }
        return true;
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .flatten()
            .map(|line| {
                let report = line
                    .split(" ")
                    .flat_map(|s| s.parse::<i32>())
                    .collect_vec();
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
            .flatten()
            .map(|line| {
                let report = line
                    .split(" ")
                    .flat_map(|s| s.parse::<i32>())
                    .collect_vec();

                if check_report_orig(&report) {
                    return 1
                }
                for i in 0..report.len() {
                    let mut changed_report = report.clone();
                    changed_report.remove(i);
                    // println!("{}: {:?}", i, changed_report);
                    if check_report_orig(&changed_report) {
                        return 1
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