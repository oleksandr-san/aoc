use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;

const DAY: &str = "6";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

#[derive(Debug, Clone)]
struct TileMap {
    pub tiles: HashMap<Position, char>,
    pub area: Rectangle,
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
                        tiles.insert((i, j), c);
                    }
                    max_i = max_i.max(i);
                    max_j = max_j.max(j);
                })
            });

        Ok(TileMap {
            tiles,
            area: ((0, 0), (max_i, max_j)),
        })
    }

    pub fn find_player(&self) -> Option<(Position, Direction)> {
        for (pos, tile) in self.tiles.iter() {
            if let Some(dir) = Direction::from_symbol(*tile) {
                return Some((*pos, dir));
            }
        }
        None
    }

    pub fn in_bounds(&self, pos: Position) -> bool {
        pos.0 >= self.area.0 .0
            && pos.1 >= self.area.0 .1
            && pos.0 <= self.area.1 .0
            && pos.1 <= self.area.1 .1
    }
}

fn walk(map: &TileMap, mut pos: Position, mut dir: Direction) -> (usize, bool) {
    let step = 1;

    let mut visited_tiles = HashSet::new();
    visited_tiles.insert(pos);

    let mut visited_directions = HashSet::new();
    visited_directions.insert((pos, dir));

    // println!("Start walking pos={:?}, dir={:?}", pos, dir);

    loop {
        let Some(new_pos) = leap(pos, dir, step) else {
            // println!("Leaped out of bounds after {} steps at pos={:?}", visited_tiles.len(), pos);
            return (visited_tiles.len(), false);
        };
        // println!("Leaping at new_pos={:?}, dir={:?}, dist={}", new_pos, dir, visited_tiles.len());

        if !map.in_bounds(new_pos) {
            // println!("Leaped out of bounds after {} steps at pos={:?}", visited_tiles.len(), pos);
            return (visited_tiles.len(), false);
        } else if visited_directions.contains(&(new_pos, dir)) {
            return (visited_tiles.len(), true);
        } else if map.tiles.contains_key(&new_pos) {
            dir = dir.turn_right();
            // println!("Turn right at pos={:?}, dir={:?}, dist={}", pos, dir, visited_tiles.len());
        } else {
            pos = new_pos;
            visited_tiles.insert(pos);
            visited_directions.insert((pos, dir));
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let space = '.';
        let mut map = TileMap::read(reader, space)?;
        let (pos, dir) = map.find_player().expect("player not found");
        map.tiles.remove(&pos);
        let (total_dist, _) = walk(&map, pos, dir);
        Ok(total_dist)
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    // region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let space = '.';
        let mut map = TileMap::read(reader, space)?;
        let (pos, dir) = map.find_player().expect("player not found");
        map.tiles.remove(&pos);

        println!("{:?}", map.area);
        let answer = (0..=map.area.1 .0)
            .into_par_iter()
            .map(|i| {
                (0..=map.area.1 .1)
                    .into_par_iter()
                    .filter(|j| !map.tiles.contains_key(&(i, *j)))
                    .filter(|j| {
                        let mut new_map = map.clone();
                        new_map.tiles.insert((i, *j), '#');

                        let (_, is_looping) = walk(&new_map, pos, dir);
                        if is_looping {
                            println!("{:?} is looping", (i, *j));
                        }
                        is_looping
                    })
                    .count()
            })
            .sum();
        Ok(answer)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{walk, TileMap};
    use std::io::BufReader;

    const LOOP1: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#.#^.....
........#.
#.........
......#..."#;

    const LOOP2: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
......#.#.
#.........
......#..."#;

    const LOOP3: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
.......##.
#.........
......#..."#;

    const LOOP4: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
##........
......#..."#;

    const LOOP5: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#..#......
......#..."#;

    const LOOP6: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#..#......
......##.."#;

    #[test]
    fn test_walk() {
        let space = '.';
        for (i, input) in [LOOP1, LOOP2, LOOP3, LOOP4, LOOP5, LOOP6]
            .iter()
            .enumerate()
        {
            let mut map =
                TileMap::read(BufReader::new(input.as_bytes()), space).expect("parse error");
            let (pos, dir) = map.find_player().expect("player not found");
            map.tiles.remove(&pos);

            let (_, is_looping) = walk(&map, pos, dir);
            assert!(is_looping, "Loop is not detected for {}", i + 1);
        }
    }
}
