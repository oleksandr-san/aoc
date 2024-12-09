use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;

const DAY: &str = "9";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = r#"2333133121414131402"#;

#[derive(Debug, Clone, Copy)]
enum DiskEntry {
    Block { file_id: u16, len: u8 },
    Free { len: u8 },
}

#[derive(Debug, Default)]
struct DiskMap {
    entries: Vec<DiskEntry>,
}

impl std::fmt::Display for DiskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let repr = self
            .entries
            .iter()
            .map(|entry| match entry {
                DiskEntry::Block { file_id, len } => file_id.to_string().repeat(*len as usize),
                DiskEntry::Free { len } => ".".repeat(*len as usize),
            })
            .join("");
        write!(f, "{}", repr)
    }
}
impl DiskMap {
    pub fn from_compact_str(line: &str) -> Self {
        let mut file_id = 0;
        let entries = line
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let d = c.to_digit(10).unwrap_or_default();
                if i % 2 != 0 {
                    DiskEntry::Free { len: d as u8 }
                } else {
                    let entry = DiskEntry::Block {
                        file_id,
                        len: d as u8,
                    };
                    file_id += 1;
                    entry
                }
            })
            .collect_vec();
        DiskMap { entries }
    }

    pub fn checksum(&self) -> usize {
        let mut block_id = 0usize;
        self.entries
            .iter()
            .cloned()
            .map(|entry| match entry {
                DiskEntry::Block { file_id, len } => {
                    let block_checksum: usize = (0usize..len as usize)
                        .map(|i| (block_id + i) * file_id as usize)
                        .sum();
                    block_id += len as usize;
                    block_checksum
                }
                DiskEntry::Free { len } => {
                    block_id += len as usize;
                    0usize
                }
            })
            .sum()
    }

    pub fn compacted(mut self) -> Self {
        let mut compacted_entries = vec![];
        let mut entry_idx = 0;
        let mut last_entry_idx = self.entries.len() - 1;

        while entry_idx <= last_entry_idx {
            let mid_idx = entry_idx + 1;
            let (first_entries, last_entries) = self.entries.split_at_mut(mid_idx);

            let entry = first_entries
                .get_mut(entry_idx)
                .expect("retrieve entry at index");
            match entry {
                DiskEntry::Block { file_id, len } if *len > 0 => {
                    compacted_entries.push(DiskEntry::Block {
                        file_id: *file_id,
                        len: *len,
                    });
                    entry_idx += 1;
                }
                DiskEntry::Free {
                    len: ref mut free_len,
                } if *free_len > 0 => {
                    while last_entry_idx >= mid_idx {
                        match last_entries
                            .get_mut(last_entry_idx - mid_idx)
                            .expect("retrieve entry at index")
                        {
                            DiskEntry::Free { .. } => {
                                last_entry_idx -= 1;
                            }
                            DiskEntry::Block {
                                file_id: last_file_id,
                                len: last_len,
                            } => {
                                if free_len >= last_len {
                                    compacted_entries.push(DiskEntry::Block {
                                        file_id: *last_file_id,
                                        len: *last_len,
                                    });
                                    *free_len -= *last_len;
                                    *last_len = 0;
                                    last_entry_idx -= 1;
                                    if *free_len == 0 {
                                        break;
                                    }
                                } else {
                                    compacted_entries.push(DiskEntry::Block {
                                        file_id: *last_file_id,
                                        len: *free_len,
                                    });
                                    *last_len -= *free_len;
                                    *free_len = 0;
                                    break;
                                }
                            }
                        }
                    }

                    if *free_len == 0 || entry_idx == last_entry_idx {
                        entry_idx += 1;
                    }
                }
                _ => {
                    entry_idx += 1;
                }
            }
        }

        DiskMap {
            entries: compacted_entries,
        }
    }

    pub fn defrag_compacted(mut self) -> Self {
        for i in (0..self.entries.len()).rev() {
            let DiskEntry::Block { len, .. } = self.entries[i] else {
                continue;
            };

            for j in 0..i {
                let DiskEntry::Free { len: free_len } = self.entries[j] else {
                    continue;
                };

                match len.cmp(&free_len) {
                    Ordering::Less => {
                        self.entries[j] = DiskEntry::Free { len };
                        self.entries.swap(i, j);
                        self.entries.insert(
                            j + 1,
                            DiskEntry::Free {
                                len: free_len - len,
                            },
                        );
                        break;
                    }
                    Ordering::Equal => {
                        self.entries.swap(i, j);
                        break;
                    }
                    Ordering::Greater => {}
                }
            }
        }

        DiskMap {
            entries: self.entries,
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .next()
            .expect("empty input")
            .map(|line| DiskMap::from_compact_str(&line).compacted().checksum())
            .expect("no solution");
        Ok(answer)
    }

    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    // region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .next()
            .expect("empty input")
            .map(|line| {
                DiskMap::from_compact_str(&line)
                    .defrag_compacted()
                    .checksum()
            })
            .expect("no solution");
        Ok(answer)
    }

    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{DiskEntry, DiskMap};

    #[test]
    fn test_disk_map_from_str() {
        let dm = DiskMap::from_compact_str("2333133121414131402");
        assert_eq!(dm.to_string(), "00...111...2...333.44.5555.6666.777.888899");

        let dm = DiskMap::from_compact_str("12345");
        assert_eq!(dm.to_string(), "0..111....22222");
    }

    #[test]
    fn test_disk_map_checksum() {
        let mut dm = DiskMap::default();
        dm.entries.push(DiskEntry::Block { file_id: 0, len: 2 });
        dm.entries.push(DiskEntry::Block { file_id: 9, len: 2 });
        dm.entries.push(DiskEntry::Block { file_id: 8, len: 1 });
        assert_eq!(dm.checksum(), 18 + 27 + 32);
    }

    #[test]
    fn test_disk_map_compact() {
        let dm = DiskMap::from_compact_str("12345");
        assert_eq!(dm.compacted().to_string(), "022111222");

        let dm = DiskMap::from_compact_str("2333133121414131402");
        assert_eq!(dm.compacted().to_string(), "0099811188827773336446555566");
    }
}
