use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

struct Garden {
    pub plots: Vec<Vec<char>>,
    pub area: Rectangle,
}

impl Garden {
    pub fn read<R: BufRead>(input: R) -> Result<Self> {
        let mut max_i = 0;
        let mut max_j = 0;

        let plots = input
            .lines()
            .map_while(Result::ok)
            .enumerate()
            .map(|(i, line)| {
                max_i = max_i.max(i);
                max_j = max_j.max(line.len() - 1);

                line.chars().collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(Self {
            plots,
            area: ((0, 0), (max_i, max_j)),
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
struct Region {
    plot: char,
    area: usize,
    perimeter: usize,
    sides: usize,
}

impl Region {
    pub fn price(&self) -> usize {
        self.area * self.perimeter
    }

    pub fn discounted_price(&self) -> usize {
        self.area * self.sides
    }
}

fn collect_regions(garden: &Garden) -> Vec<Region> {
    let mut region_indices = garden
        .plots
        .iter()
        .map(|row| std::iter::repeat(None).take(row.len()).collect_vec())
        .collect::<Vec<_>>();

    let mut queue = VecDeque::new();
    let mut regions = Vec::new();

    for (i, row) in garden.plots.iter().enumerate() {
        for (j, plot) in row.iter().enumerate() {
            if region_indices[i][j].is_some() {
                continue;
            }

            queue.clear();
            queue.push_front((i, j));
            let mut region = Region {
                plot: *plot,
                ..Default::default()
            };
            while let Some(pos) = queue.pop_front() {
                if region_indices[pos.0][pos.1].is_some() {
                    continue;
                }
                region.area += 1;
                region_indices[pos.0][pos.1] = Some(regions.len());

                for dir in [Direction::N, Direction::E, Direction::W, Direction::S] {
                    if let Some(next_pos) = leap_in_bounds(pos, dir, 1, &garden.area) {
                        let next_plot = garden.plots[next_pos.0][next_pos.1];
                        if next_plot != *plot {
                            region.perimeter += 1;
                        } else if region_indices[next_pos.0][next_pos.1].is_none() {
                            queue.push_back(next_pos);
                        }
                    } else {
                        region.perimeter += 1;
                    }
                }
            }

            regions.push(region);
        }
    }

    // Calculate region sides
    for (i, row) in garden.plots.iter().enumerate() {
        for (j, plot) in row.iter().enumerate() {
            let Some(region_idx) = region_indices[i][j] else {
                continue;
            };
            let Some(region) = regions.get_mut(region_idx) else {
                continue;
            };

            let pos = (i, j);
            for dir in [Direction::N, Direction::E, Direction::W, Direction::S] {
                let p1 = leap_in_bounds(pos, dir, 1, &garden.area).map(|(x, y)| garden.plots[x][y]);
                let p2 = leap_in_bounds(pos, dir.turn_right(), 1, &garden.area)
                    .map(|(x, y)| garden.plots[x][y]);

                match (p1, p2) {
                    (Some(p1), Some(p2)) if p2 != *plot && p1 != *plot => {
                        region.sides += 1;
                    }
                    (Some(p1), None) if p1 != *plot => {
                        region.sides += 1;
                    }
                    (None, Some(p2)) if p2 != *plot => {
                        region.sides += 1;
                    }
                    (None, None) => {
                        region.sides += 1;
                    }

                    (Some(p1), Some(p2)) if p2 == *plot && p1 == *plot => {
                        let p3 = leap_in_bounds(pos, dir.turn_45_deg(), 1, &garden.area)
                            .map(|(x, y)| garden.plots[x][y]);
                        match p3 {
                            Some(p3) if p3 != *plot => {
                                region.sides += 1;
                            }
                            None => {
                                region.sides += 1;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    regions
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let garden = Garden::read(reader)?;
        let answer = collect_regions(&garden)
            .iter()
            .map(Region::price)
            .sum::<usize>();
        Ok(answer)
    }

    assert_eq!(1930, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    // region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let garden = Garden::read(reader)?;
        let answer = collect_regions(&garden)
            .iter()
            .map(Region::discounted_price)
            .sum::<usize>();
        Ok(answer)
    }

    assert_eq!(1206, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{collect_regions, Garden, Region};

    #[test]
    fn test_collect_regions_case1() {
        let garden = Garden::read(
            r#"AAAA
BBCD
BBCC
EEEC"#
                .as_bytes(),
        )
        .unwrap();
        let regions = collect_regions(&garden);
        assert_eq!(
            regions,
            vec![
                Region {
                    plot: 'A',
                    area: 4,
                    perimeter: 10,
                    sides: 4,
                },
                Region {
                    plot: 'B',
                    area: 4,
                    perimeter: 8,
                    sides: 4,
                },
                Region {
                    plot: 'C',
                    area: 4,
                    perimeter: 10,
                    sides: 8,
                },
                Region {
                    plot: 'D',
                    area: 1,
                    perimeter: 4,
                    sides: 4,
                },
                Region {
                    plot: 'E',
                    area: 3,
                    perimeter: 8,
                    sides: 4,
                }
            ]
        );

        assert_eq!(regions.iter().map(Region::price).sum::<usize>(), 140,);
        assert_eq!(
            regions.iter().map(Region::discounted_price).sum::<usize>(),
            80,
        );
    }

    #[test]
    fn test_collect_regions_case2() {
        let garden = Garden::read(
            r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#
                .as_bytes(),
        )
        .unwrap();

        let regions = collect_regions(&garden);
        assert_eq!(
            regions.iter().map(Region::discounted_price).sum::<usize>(),
            436,
        );
    }

    #[test]
    fn test_collect_regions_case3() {
        let garden = Garden::read(
            r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#
                .as_bytes(),
        )
        .unwrap();

        let regions = collect_regions(&garden);
        assert_eq!(
            regions,
            vec![
                Region {
                    plot: 'E',
                    area: 17,
                    perimeter: 36,
                    sides: 12
                },
                Region {
                    plot: 'X',
                    area: 4,
                    perimeter: 10,
                    sides: 4
                },
                Region {
                    plot: 'X',
                    area: 4,
                    perimeter: 10,
                    sides: 4
                },
            ],
        );
        assert_eq!(
            regions.iter().map(Region::discounted_price).sum::<usize>(),
            236,
        );
    }

    #[test]
    fn test_collect_regions_case4() {
        let garden = Garden::read(
            r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#
                .as_bytes(),
        )
        .unwrap();

        let regions = collect_regions(&garden);
        assert_eq!(
            regions.iter().map(Region::discounted_price).sum::<usize>(),
            368,
        );
    }

    #[test]
    fn test_collect_regions_case5() {
        let garden = Garden::read(
            r#"OOOOO
OXOXO
OXXXO"#
                .as_bytes(),
        )
        .unwrap();

        let regions = collect_regions(&garden);
        assert_eq!(
            regions.iter().map(Region::discounted_price).sum::<usize>(),
            160,
        );
    }

    #[test]
    fn test_collect_regions_case6() {
        let garden = Garden::read(
            r#".....
.AAA.
.A.A.
.AA..
.A.A.
.AAA.
....."#
                .as_bytes(),
        )
        .unwrap();

        let regions = collect_regions(&garden);
        assert_eq!(regions.iter().map(Region::price).sum::<usize>(), 1202,);
        assert_eq!(
            regions,
            vec![
                Region {
                    plot: '.',
                    area: 21,
                    perimeter: 42,
                    sides: 12,
                },
                Region {
                    plot: 'A',
                    area: 12,
                    perimeter: 26,
                    sides: 16,
                },
                Region {
                    plot: '.',
                    area: 1,
                    perimeter: 4,
                    sides: 4
                },
                Region {
                    plot: '.',
                    area: 1,
                    perimeter: 4,
                    sides: 4
                },
            ]
        );
        assert_eq!(
            regions.iter().map(Region::discounted_price).sum::<usize>(),
            452,
        );
    }
}
