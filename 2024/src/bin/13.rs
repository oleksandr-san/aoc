use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;

const DAY: &str = "13";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

#[derive(Debug, Default, PartialEq)]
struct ClawMachine {
    a: (usize, usize),
    b: (usize, usize),
    prize: Position,
}

fn read_claw_machines<R: BufRead>(input: R) -> Result<Vec<ClawMachine>> {
    let mut claw_machines = Vec::new();
    let mut current_machine = None;

    input
        .lines()
        .map_while(Result::ok)
        .filter(|i| !i.is_empty())
        .for_each(|line| match line.split_once(": ") {
            Some(("Button A", data)) => {
                if let Some((x, y)) = data.split_once(", ") {
                    let x = x[2..].parse::<usize>().expect("number coordinate");
                    let y = y[2..].parse::<usize>().expect("number coordinate");
                    current_machine = Some(ClawMachine {
                        a: (x, y),
                        ..Default::default()
                    });
                }
            }
            Some(("Button B", data)) => {
                if let Some((x, y)) = data.split_once(", ") {
                    let x = x[2..].parse::<usize>().expect("number coordinate");
                    let y = y[2..].parse::<usize>().expect("number coordinate");
                    if let Some(machine) = current_machine.as_mut() {
                        machine.b = (x, y);
                    }
                }
            }
            Some(("Prize", data)) => {
                if let Some((x, y)) = data.split_once(", ") {
                    let x = x[2..].parse::<usize>().expect("number coordinate");
                    let y = y[2..].parse::<usize>().expect("number coordinate");
                    if let Some(mut machine) = current_machine.take() {
                        machine.prize = (x, y);
                        claw_machines.push(machine);
                    }
                }
            }
            _ => {}
        });

    Ok(claw_machines)
}

fn find_winning_combination(machine: &ClawMachine) -> Option<((usize, usize), usize)> {
    let b_cf = machine.b.0 as f64 / machine.b.1 as f64;
    let a_cf = machine.a.0 as f64 - machine.a.1 as f64 * b_cf;
    let x = (machine.prize.0 as f64 - machine.prize.1 as f64 * b_cf) / a_cf;
    let y = (machine.prize.0 as f64 - machine.a.0 as f64 * x) / machine.b.0 as f64;

    for a in [x.floor() as usize, x.ceil() as usize] {
        for b in [y.floor() as usize, y.ceil() as usize] {
            let px = a * machine.a.0 + b * machine.b.0;
            let py = a * machine.a.1 + b * machine.b.1;
            if (px, py) != machine.prize {
                continue;
            }
            let cost = a * 3 + b;
            return Some(((a, b), cost));
        }
    }
    None
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer = read_claw_machines(reader)?
            .iter()
            .filter_map(find_winning_combination)
            .map(|(_, cost)| cost)
            .sum();
        Ok(answer)
    }

    assert_eq!(480, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    // region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let delta = 10000000000000;
        let machines = read_claw_machines(reader)?
            .into_iter()
            .map(|mut machine| {
                machine.prize = (machine.prize.0 + delta, machine.prize.1 + delta);
                machine
            })
            .collect::<Vec<_>>();
        let answer = machines
            .iter()
            .filter_map(find_winning_combination)
            .map(|(_, cost)| cost)
            .sum();
        Ok(answer)
    }

    assert_eq!(875318608908, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{find_winning_combination, read_claw_machines, ClawMachine, TEST};
    use std::io::BufReader;

    #[test]
    fn test_read_claw_machines() {
        let machines = read_claw_machines(BufReader::new(TEST.as_bytes())).unwrap();
        assert_eq!(
            machines,
            vec![
                ClawMachine {
                    a: (94, 34),
                    b: (22, 67),
                    prize: (8400, 5400)
                },
                ClawMachine {
                    a: (26, 66),
                    b: (67, 21),
                    prize: (12748, 12176)
                },
                ClawMachine {
                    a: (17, 86),
                    b: (84, 37),
                    prize: (7870, 6450)
                },
                ClawMachine {
                    a: (69, 23),
                    b: (27, 71),
                    prize: (18641, 10279)
                },
            ]
        );
    }

    #[test]
    fn test_find_winning_combination() {
        let comb = find_winning_combination(&ClawMachine {
            a: (94, 34),
            b: (22, 67),
            prize: (8400, 5400),
        });
        assert_eq!(comb, Some(((80, 40), 280)));
    }
}
