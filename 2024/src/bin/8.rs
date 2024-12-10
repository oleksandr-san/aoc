use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;

const DAY: &str = "8";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

type AbsolutePosition = (isize, isize);
type AbsoluteRectangle = (AbsolutePosition, AbsolutePosition);

fn rectangle_includes(rectangle: AbsoluteRectangle, pos: AbsolutePosition) -> bool {
    pos.0 >= rectangle.0 .0
        && pos.1 >= rectangle.0 .1
        && pos.0 <= rectangle.1 .0
        && pos.1 <= rectangle.1 .1
}

#[derive(Debug, Clone)]
struct TileMap {
    pub tiles: HashMap<AbsolutePosition, char>,
    pub area: AbsoluteRectangle,
}

impl TileMap {
    pub fn read<R: BufRead>(input: R, space: char) -> Result<TileMap> {
        let mut tiles = HashMap::new();
        let mut max_i = 0;
        let mut max_j = 0;

        input
            .lines()
            .map_while(Result::ok)
            .enumerate()
            .for_each(|(i, line)| {
                line.chars().enumerate().for_each(|(j, c)| {
                    if c != space {
                        tiles.insert((i as isize, j as isize), c);
                    }
                    max_i = max_i.max(i);
                    max_j = max_j.max(j);
                })
            });

        Ok(TileMap {
            tiles,
            area: ((0, 0), (max_i as isize, max_j as isize)),
        })
    }

    pub fn in_bounds(&self, pos: AbsolutePosition) -> bool {
        rectangle_includes(self.area, pos)
    }
}

fn calc_antinode_locations(s1: AbsolutePosition, s2: AbsolutePosition) -> Vec<AbsolutePosition> {
    let dx = s2.0 - s1.0;
    let dy = s2.1 - s1.1;

    let locations = vec![(s1.0 - dx, s1.1 - dy), (s2.0 + dx, s2.1 + dy)];
    locations
}

fn calculate_line_points_in_area(
    s1: AbsolutePosition,
    s2: AbsolutePosition,
    area: &AbsoluteRectangle,
) -> Vec<AbsolutePosition> {
    let (x1, y1) = s1;
    let (x2, y2) = s2;

    let a = y2 - y1;
    let b = x1 - x2;
    let c = x2 * y1 - x1 * y2;

    let (min_x, min_y) = (area.0 .0, area.0 .1);
    let (max_x, max_y) = (area.1 .0, area.1 .1);

    let mut points = Vec::new();
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            // Check if the point (x, y) satisfies the line equation
            if a * x + b * y + c == 0 {
                points.push((x, y));
            }
        }
    }

    points
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let map = TileMap::read(reader, '.')?;

        let mut stations_by_name: HashMap<char, Vec<_>> = HashMap::new();
        map.tiles.iter().for_each(|(p, c)| {
            stations_by_name.entry(*c).or_default().push(*p);
        });

        let mut antinode_locations = HashSet::new();
        stations_by_name.iter().for_each(|(_, ps)| {
            for i in 0..ps.len() {
                for j in 0..ps.len() {
                    if i == j {
                        continue;
                    }

                    calc_antinode_locations(ps[i], ps[j])
                        .into_iter()
                        .filter(|a| map.in_bounds(*a))
                        .for_each(|a| {
                            antinode_locations.insert(a);
                        });
                }
            }
        });

        let answer = antinode_locations.len();
        Ok(answer)
    }

    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    // region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let map = TileMap::read(reader, '.')?;

        let mut stations_by_name: HashMap<char, Vec<_>> = HashMap::new();
        map.tiles.iter().for_each(|(p, c)| {
            stations_by_name.entry(*c).or_default().push(*p);
        });

        let mut antinode_locations = HashSet::new();
        stations_by_name.iter().for_each(|(_, ps)| {
            for i in 0..ps.len() {
                for j in 0..ps.len() {
                    if i == j {
                        continue;
                    }

                    antinode_locations
                        .extend(calculate_line_points_in_area(ps[i], ps[j], &map.area));
                }
            }
        });

        let answer = antinode_locations.len();
        Ok(answer)
    }

    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_antinode_locations() {
        assert_eq!(
            calc_antinode_locations((3, 4), (5, 5)),
            vec![(1, 3), (7, 6)]
        );
        assert_eq!(
            calc_antinode_locations((5, 5), (3, 4)),
            vec![(7, 6), (1, 3)]
        );

        assert_eq!(
            calc_antinode_locations((3, 4), (4, 8)),
            vec![(2, 0), (5, 12)]
        );
        assert_eq!(
            calc_antinode_locations((4, 8), (3, 4)),
            vec![(5, 12), (2, 0)]
        );

        assert_eq!(
            calc_antinode_locations((4, 8), (5, 5)),
            vec![(3, 11), (6, 2)]
        );
        assert_eq!(
            calc_antinode_locations((5, 5), (4, 8)),
            vec![(6, 2), (3, 11)]
        );
    }

    #[test]
    fn test_calculate_line_points_in_area() {
        let area = ((0, 0), (9, 9));
        assert_eq!(
            calculate_line_points_in_area((0, 0), (2, 1), &area),
            vec![(0, 0), (2, 1), (4, 2), (6, 3), (8, 4)]
        );
        assert_eq!(
            calculate_line_points_in_area((2, 1), (0, 0), &area),
            vec![(0, 0), (2, 1), (4, 2), (6, 3), (8, 4)]
        );

        assert_eq!(
            calculate_line_points_in_area((0, 0), (1, 3), &area),
            vec![(0, 0), (1, 3), (2, 6), (3, 9)]
        );
        assert_eq!(
            calculate_line_points_in_area((1, 3), (0, 0), &area),
            vec![(0, 0), (1, 3), (2, 6), (3, 9)]
        );

        assert_eq!(
            calculate_line_points_in_area((1, 3), (2, 1), &area),
            vec![(0, 5), (1, 3), (2, 1)]
        );
        assert_eq!(
            calculate_line_points_in_area((2, 1), (1, 3), &area),
            vec![(0, 5), (1, 3), (2, 1)]
        );
    }
}
