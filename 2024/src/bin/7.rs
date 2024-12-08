use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;

const DAY: &str = "7";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    Mul,
    Add,
    Concat,
}

fn concat(a: usize, b: usize) -> usize {
    a * 10usize.pow(b.ilog10() + 1) + b
}

fn calculate(arguments: &[usize], operations: &[Operation]) -> usize {
    let mut result = arguments[0];

    for i in 0..operations.len() {
        let arg = arguments[i + 1];
        let op = &operations[i];

        match op {
            Operation::Mul => {
                result *= arg;
            }
            Operation::Add => {
                result += arg;
            }
            Operation::Concat => {
                result = concat(result, arg);
            }
        }
    }

    result
}

fn product_recursive<T: Clone>(
    alphabet: &[T],
    n: usize,
    current: &mut Vec<T>,
    result: &mut Vec<Vec<T>>,
) {
    if current.len() == n {
        result.push(current.iter().cloned().collect());
        return;
    }

    for c in alphabet.iter() {
        current.push(c.clone());
        product_recursive(alphabet, n, current, result);
        current.pop();
    }
}

fn product<T: Clone>(alphabet: &[T], n: usize) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    product_recursive(alphabet, n, &mut current, &mut result);
    result
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .flatten()
            .filter_map(|line| {
                let (result, args) = line.split_once(": ")?;
                let result = result.parse::<usize>().ok()?;
                let args = args
                    .split_whitespace()
                    .flat_map(|a| a.parse::<usize>().ok())
                    .collect_vec();

                let op_alphabet = vec![Operation::Add, Operation::Mul];
                for ops in product(&op_alphabet, args.len() - 1) {
                    let ops_result = calculate(&args, &ops);
                    if ops_result == result {
                        return Some(result);
                    }
                }
                None
            })
            .sum::<usize>();
        Ok(answer)
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    // region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .flatten()
            .filter_map(|line| {
                let (result, args) = line.split_once(": ")?;
                let result = result.parse::<usize>().ok()?;
                let args = args
                    .split_whitespace()
                    .flat_map(|a| a.parse::<usize>().ok())
                    .collect_vec();

                let op_alphabet = vec![Operation::Add, Operation::Mul, Operation::Concat];
                for ops in product(&op_alphabet, args.len() - 1) {
                    let ops_result = calculate(&args, &ops);
                    if ops_result == result {
                        return Some(result);
                    }
                }
                None
            })
            .sum::<usize>();
        Ok(answer)
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{calculate, Operation};

    #[test]
    fn test_calculate() {
        assert_eq!(calculate(&[12, 345], &[Operation::Concat]), 12345);

        assert_eq!(
            calculate(&[17, 8, 14], &[Operation::Concat, Operation::Add]),
            192,
        );

        assert_eq!(
            calculate(
                &[6, 8, 6, 15],
                &[Operation::Mul, Operation::Concat, Operation::Mul]
            ),
            7290,
        );
    }
}
