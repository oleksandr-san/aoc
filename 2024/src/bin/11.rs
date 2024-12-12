use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = r#"125 17"#;

fn count_digits(mut n: usize) -> usize {
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count
}

fn split_in_half(mut n: usize, n_digits: usize) -> (usize, usize) {
    let mut m = 0;

    for i in 0..n_digits / 2 {
        let r = n % 10;
        m += r * 10_usize.pow(i as u32);
        n /= 10;
    }

    (n, m)
}

fn blink(stones: impl Iterator<Item = usize>, blinks: usize) -> u128 {
    let mut calculator = Calculator::new();
    stones
        .into_iter()
        .map(|stone| calculator.calculate(stone, blinks))
        .sum()
}

struct Calculator {
    cache: HashMap<(usize, usize), u128>,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn calculate(&mut self, stone: usize, blinks: usize) -> u128 {
        if blinks == 0 {
            return 1;
        }
        let key = (stone, blinks);
        if let Some(count) = self.cache.get(&key) {
            return *count;
        }

        let count = if stone == 0 {
            self.calculate(1, blinks - 1)
        } else {
            let n_digits = count_digits(stone);
            if n_digits % 2 == 0 {
                let (n1, n2) = split_in_half(stone, n_digits);
                self.calculate(n1, blinks - 1) + self.calculate(n2, blinks - 1)
            } else {
                self.calculate(stone * 2024, blinks - 1)
            }
        };

        self.cache.insert(key, count);
        count
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u128> {
        let answer = blink(
            reader
                .lines()
                .next()
                .expect("read line")?
                .split_whitespace()
                .filter_map(|x| x.parse::<usize>().ok()),
            25,
        );
        Ok(answer)
    }

    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    // region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u128> {
        let answer = blink(
            reader
                .lines()
                .next()
                .expect("read line")?
                .split_whitespace()
                .filter_map(|x| x.parse::<usize>().ok()),
            75,
        );
        Ok(answer)
    }

    assert_eq!(22938365706844, part2(BufReader::new("0".as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{blink, count_digits, split_in_half};

    #[test]
    fn test_count_digits() {
        assert_eq!(count_digits(10), 2);
        assert_eq!(count_digits(999), 3);
        assert_eq!(count_digits(2097446912), 10);
    }

    #[test]
    fn test_split_in_half() {
        assert_eq!(split_in_half(99, 2), (9, 9));
        assert_eq!(split_in_half(2097446912, 10), (20974, 46912));
        assert_eq!(split_in_half(253000, 6), (253, 000));
    }

    #[test]
    fn test_blink() {
        let initial: Vec<usize> = vec![125, 17];

        assert_eq!(blink(initial.clone().into_iter(), 1), 3);
        assert_eq!(blink(initial.clone().into_iter(), 2), 4);
        assert_eq!(blink(initial.clone().into_iter(), 3), 5);
    }
}
