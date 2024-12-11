use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

struct TopographyMap {
    pub topography: Vec<Vec<u8>>,
    pub area: Rectangle,
}

impl TopographyMap {
    pub fn read<R: BufRead>(input: R) -> Result<Self> {
        let mut max_i = 0;
        let mut max_j = 0;

        let topography = input
            .lines()
            .map_while(Result::ok)
            .enumerate()
            .map(|(i, line)| {
                max_i = max_i.max(i);
                max_j = max_j.max(line.len() - 1);

                line.chars()
                    .map(|c| {
                        if c as u8 >= b'0' {
                            c as u8 - b'0'
                        } else {
                            c as u8
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(Self {
            topography,
            area: ((0, 0), (max_i, max_j)),
        })
    }

    pub fn score_trails<F: FnMut(&TopographyMap, Position) -> usize>(&self, mut f: F) -> usize {
        self.topography
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &c)| c == 0)
                    .map(|(j, _)| f(self, (i, j)))
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
}

fn score_trail(map: &TopographyMap, start: Position) -> usize {
    let mut score = 0;
    let mut queue = VecDeque::from([start]);
    let mut visited = HashSet::new();

    while let Some(pos) = queue.pop_front() {
        let height = map.topography[pos.0][pos.1];
        if height == 9 {
            score += 1;
            continue;
        }

        for dir in [Direction::E, Direction::W, Direction::S, Direction::N] {
            let Some(new_pos) = leap(pos, dir, 1) else {
                continue;
            };
            if !rectangle_includes(&map.area, new_pos) || visited.contains(&new_pos) {
                continue;
            }
            let new_height = map.topography[new_pos.0][new_pos.1];

            if new_height == height + 1 {
                queue.push_back(new_pos);
                visited.insert(new_pos);
            }
        }
    }

    score
}

fn score_trail_v2(map: &TopographyMap, start: Position) -> usize {
    let mut score = 0;
    let mut queue = VecDeque::from([start]);

    while let Some(pos) = queue.pop_front() {
        let height = map.topography[pos.0][pos.1];
        if height == 9 {
            score += 1;
            continue;
        }

        for dir in [Direction::E, Direction::W, Direction::S, Direction::N] {
            let Some(new_pos) = leap(pos, dir, 1) else {
                continue;
            };
            if !rectangle_includes(&map.area, new_pos) {
                continue;
            }
            let new_height = map.topography[new_pos.0][new_pos.1];

            if new_height == height + 1 {
                queue.push_back(new_pos);
            }
        }
    }

    score
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let map = TopographyMap::read(reader)?;
        let answer = map.score_trails(score_trail);
        Ok(answer)
    }

    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    // region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let map = TopographyMap::read(reader)?;
        let answer = map.score_trails(score_trail_v2);
        Ok(answer)
    }

    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{score_trail, score_trail_v2, TopographyMap};

    #[test]
    fn test_score_trail() {
        let map = TopographyMap::read(
            r#"0123
1234
8765
9876"#
                .as_bytes(),
        )
        .unwrap();

        assert_eq!(map.area, ((0, 0), (3, 3)));
        assert_eq!(score_trail(&map, (0, 0)), 1);

        let map = TopographyMap::read(
            r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#
                .as_bytes(),
        )
        .unwrap();

        assert_eq!(map.area, ((0, 0), (7, 7)));
        assert_eq!(score_trail(&map, (0, 2)), 5);
    }

    #[test]
    fn test_score_trail_v2() {
        let map = TopographyMap::read(
            r#"012345
123456
234567
345678
4.6789
56789."#
                .as_bytes(),
        )
        .unwrap();

        assert_eq!(map.area, ((0, 0), (5, 5)));
        assert_eq!(score_trail_v2(&map, (0, 0)), 227);
    }
}
