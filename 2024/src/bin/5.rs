use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;

const DAY: &str = "5";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut rules: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut rules_read = false;

        let answer = reader
            .lines()
            .map_while(Result::ok)
            .filter_map(|line| {
                println!("{}", line);
                if !rules_read {
                    if line.is_empty() {
                        rules_read = true;
                    } else if let Some((a, b)) = line.split_once("|") {
                        if let (Ok(a), Ok(b)) = (a.parse::<usize>(), b.parse::<usize>()) {
                            rules.entry(a).or_default().insert(b);
                        }
                    }
                    None
                } else {
                    let update = line
                        .split(',')
                        .filter_map(|x| x.parse::<usize>().ok())
                        .collect_vec();

                    let mut seen = HashSet::new();
                    for (i, c) in update.iter().enumerate() {
                        seen.insert(*c);
                        if i == 0 {
                            continue;
                        }

                        if let Some(should_follow) = rules.get(c) {
                            if should_follow.intersection(&seen).count() != 0 {
                                println!(
                                    "Update {:?} is invalid, expected  all of {:?} to follow {}",
                                    update, should_follow, c
                                );
                                return None;
                            }
                        }
                    }
                    Some(update[update.len() / 2])
                }
            })
            .sum::<usize>();
        Ok(answer)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    // region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut rules: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut rules_read = false;

        let answer = reader
            .lines()
            .map_while(Result::ok)
            .filter_map(|line| {
                println!("{}", line);
                if !rules_read {
                    if line.is_empty() {
                        rules_read = true;
                    } else if let Some((a, b)) = line.split_once("|") {
                        if let (Ok(a), Ok(b)) = (a.parse::<usize>(), b.parse::<usize>()) {
                            rules.entry(a).or_default().insert(b);
                        }
                    }
                    None
                } else {
                    let mut update = line
                        .split(',')
                        .filter_map(|x| x.parse::<usize>().ok())
                        .collect_vec();
                    let orig_update = update.clone();

                    let mut has_changes = false;
                    let mut seen = HashSet::new();
                    for i in 0..update.len() {
                        let c = update[i];
                        seen.insert(c);
                        if i == 0 {
                            continue;
                        }

                        if let Some(should_follow) = rules.get(&c) {
                            if should_follow.intersection(&seen).count() != 0 {
                                let mut k = i;
                                for j in (0..i).rev() {
                                    let d = update[j];
                                    if should_follow.contains(&d) {
                                        update.swap(k, j);
                                        k -= 1;
                                        println!("Swap {} to {}: {:?}", c, d, update);
                                        has_changes = true;
                                    }
                                }
                            }
                        }
                    }

                    if has_changes {
                        println!("Update {:?} becomes {:?}", orig_update, update);
                        Some(update[update.len() / 2])
                    } else {
                        None
                    }
                }
            })
            .sum::<usize>();
        Ok(answer)
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}
