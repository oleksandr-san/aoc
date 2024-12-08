use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;

const DAY: &str = "4";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let needle = ['X', 'M', 'A', 'S'];
        let first_char = needle[0];

        let text = reader
            .lines()
            .map_while(Result::ok)
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let area: Rectangle = ((0, 0), (text.len() - 1, text[0].len() - 1));

        let answer = text
            .iter()
            .enumerate()
            .map(|(i, line)| {
                line.iter()
                    .cloned()
                    .enumerate()
                    .filter(|(_, c)| *c == first_char)
                    .map(|(j, _)| {
                        Direction::iter()
                            .flat_map(|dir| beam((i, j), *dir, needle.len(), area.1))
                            .filter(|b| {
                                let found = b
                                    .iter()
                                    .cloned()
                                    .enumerate()
                                    .skip(1)
                                    .all(|(k, bp)| needle[k] == text[bp.0][bp.1]);
                                if found {
                                    println!("Found word with beam={:?}", b);
                                }
                                found
                            })
                            .count()
                    })
                    .sum::<usize>()
            })
            .sum();
        Ok(answer)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    // region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let target_char = 'A';

        let text = reader
            .lines()
            .map_while(Result::ok)
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let area: Rectangle = ((0, 0), (text.len() - 1, text[0].len() - 1));

        let answer = text
            .iter()
            .enumerate()
            .map(|(i, line)| {
                line.iter()
                    .cloned()
                    .enumerate()
                    .filter(|(_, c)| *c == target_char)
                    .flat_map(|(j, _)| {
                        let pos = (i, j);

                        let ne = beam(pos, Direction::NE, 2, area.1)?[1];
                        let sw = beam(pos, Direction::SW, 2, area.1)?[1];

                        let nw = beam(pos, Direction::NW, 2, area.1)?[1];
                        let se = beam(pos, Direction::SE, 2, area.1)?[1];

                        let d1_match = text[ne.0][ne.1] == 'M' && text[sw.0][sw.1] == 'S'
                            || text[ne.0][ne.1] == 'S' && text[sw.0][sw.1] == 'M';
                        let d2_match = text[nw.0][nw.1] == 'M' && text[se.0][se.1] == 'S'
                            || text[nw.0][nw.1] == 'S' && text[se.0][se.1] == 'M';

                        if d1_match && d2_match {
                            Some(1)
                        } else {
                            None
                        }
                    })
                    .sum::<usize>()
            })
            .sum();
        Ok(answer)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_beam() {
        use super::{beam, Direction};

        assert_eq!(
            beam((4, 0), Direction::N, 4, (4, 4)),
            Some(vec![(4, 0), (3, 0), (2, 0), (1, 0)])
        );
        assert_eq!(
            beam((0, 1), Direction::S, 4, (4, 4)),
            Some(vec![(0, 1), (1, 1), (2, 1), (3, 1)])
        );

        assert_eq!(
            beam((0, 4), Direction::W, 4, (4, 4)),
            Some(vec![(0, 4), (0, 3), (0, 2), (0, 1)])
        );
        assert_eq!(
            beam((0, 1), Direction::E, 4, (4, 4)),
            Some(vec![(0, 1), (0, 2), (0, 3), (0, 4)])
        );

        assert_eq!(
            beam((4, 4), Direction::NW, 4, (4, 4)),
            Some(vec![(4, 4), (3, 3), (2, 2), (1, 1)])
        );
        assert_eq!(
            beam((4, 0), Direction::NE, 4, (4, 4)),
            Some(vec![(4, 0), (3, 1), (2, 2), (1, 3)])
        );

        assert_eq!(
            beam((9, 3), Direction::NW, 4, (9, 9)),
            Some(vec![(9, 3), (8, 2), (7, 1), (6, 0)])
        );
    }
}
