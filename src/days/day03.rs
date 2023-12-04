use crate::{Solution, SolutionPair};
use std::{collections::BTreeSet, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

fn close_to_symbol(
    (row_idx, col_idx): (usize, usize),
    len: usize,
    symbol_locations: &Vec<(usize, usize)>,
) -> bool {
    let row_start = if row_idx == 0 { 0 } else { row_idx - 1 };
    let col_start = if col_idx == 0 { 0 } else { col_idx - 1 };

    for i in row_start..row_idx + 2 {
        for j in col_start..col_idx + len + 1 {
            if symbol_locations.iter().any(|(r, c)| (r, c) == (&i, &j)) {
                return true;
            }
        }
    }
    return false;
}

fn get_number_as_string(map: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> String {
    let row = map.get(row_idx).unwrap();

    let mut num_str = "".to_owned();

    for i in 0.. {
        if i > col_idx {
            break;
        }
        if col_idx == 0 {
            let thing = row.get(0).unwrap();
            if thing.is_digit(10) {
                num_str = format!("{thing}{num_str}");
            }
            break;
        }
        let thing = row.get(col_idx - i).unwrap();
        if thing.is_digit(10) {
            num_str = format!("{thing}{num_str}");
        } else {
            break;
        }
    }
    for i in 1.. {
        let thing = row.get(col_idx + i);
        if thing.is_none() {
            break;
        }
        let thing = thing.unwrap();
        if thing.is_digit(10) {
            num_str = format!("{num_str}{thing}");
        } else {
            break;
        }
    }

    num_str
}

fn is_number_beginning(col: &Vec<char>, idx: usize) -> bool {
    idx == 0 || !col.get(idx - 1).unwrap().is_digit(10)
}

pub fn solve() -> SolutionPair {
    let input: String = read_to_string("input/day03").unwrap();

    let rows = input.split("\n");

    let map = rows
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let symbol_locations = map
        .clone()
        .into_iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.into_iter()
                .enumerate()
                .filter_map(move |(col_idx, item)| {
                    if item != '.' && !item.is_digit(10) {
                        Some((row_idx, col_idx))
                    } else {
                        None
                    }
                })
        })
        .collect::<Vec<_>>();

    let mut total_sum = 0;

    for (row_idx, row) in map.clone().into_iter().enumerate() {
        for (col_idx, col) in row.clone().into_iter().enumerate() {
            if col.is_digit(10) && is_number_beginning(&row, col_idx) {
                let number_as_string = get_number_as_string(&map, row_idx, col_idx);
                let len = number_as_string.len();

                if close_to_symbol((row_idx, col_idx), len, &symbol_locations) {
                    total_sum += number_as_string.parse::<u64>().unwrap();
                }
            }
        }
    }

    let star_locations = map
        .clone()
        .into_iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.into_iter()
                .enumerate()
                .filter_map(move |(col_idx, item)| {
                    if item == '*' {
                        Some((row_idx, col_idx))
                    } else {
                        None
                    }
                })
        })
        .collect::<Vec<_>>();

    let star_sums = star_locations
        .into_iter()
        .map(|(row, col)| {
            let mut nums_seen = BTreeSet::new();
            let row_start = if row == 0 { 0 } else { row - 1 };
            let col_start = if col == 0 { 0 } else { col - 1 };
            let row_end = if row == map.len() { row } else { row + 2 };
            let col_end = if col == map.get(row).unwrap().len() {
                col
            } else {
                col + 2
            };

            for r in row_start..row_end {
                for c in col_start..col_end {
                    let item = map.get(r).and_then(|row| row.get(c));
                    if let Some(item) = item {
                        if item.is_digit(10) {
                            nums_seen.insert(get_number_as_string(&map, r, c));
                        }
                    }
                }
            }

            if nums_seen.len() == 2 {
                nums_seen.pop_first().unwrap().parse::<u64>().unwrap()
                    * nums_seen.pop_first().unwrap().parse::<u64>().unwrap()
            } else {
                0
            }
        })
        .sum();

    // Your solution here...
    let sol1: u64 = total_sum;
    let sol2: u64 = star_sums;

    (Solution::from(sol1), Solution::from(sol2))
}
