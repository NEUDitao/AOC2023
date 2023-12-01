use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day04").unwrap();
    let rows = input.split('\n');

    let total_completely_overlapping = rows
        .clone()
        .into_iter()
        .filter(|row| {
            let mut ranges = row
                .split(',')
                .into_iter()
                .map(|range| {
                    let mut split = range.split('-').collect::<Vec<_>>();
                    let range_x = split.remove(0);
                    let range_y = split.remove(0);
                    (
                        str::parse::<i32>(range_x).unwrap(),
                        str::parse::<i32>(range_y).unwrap(),
                    )
                })
                .collect::<Vec<_>>();
            let (range_1_x, range_1_y) = ranges.remove(0);
            let (range_2_x, range_2_y) = ranges.remove(0);
            (range_1_x <= range_2_x && range_1_y >= range_2_y)
                || (range_2_x <= range_1_x && range_1_y <= range_2_y)
        })
        .count();

    let any_overlapping = rows
        .clone()
        .into_iter()
        .filter(|row| {
            let mut ranges = row
                .split(',')
                .into_iter()
                .map(|range| {
                    let mut split = range.split('-').collect::<Vec<_>>();
                    let range_x = split.remove(0);
                    let range_y = split.remove(0);
                    (
                        str::parse::<i32>(range_x).unwrap(),
                        str::parse::<i32>(range_y).unwrap(),
                    )
                })
                .collect::<Vec<_>>();
            let (range_1_x, range_1_y) = ranges.remove(0);
            let (range_2_x, range_2_y) = ranges.remove(0);
            range_1_x <= range_2_y && range_1_y >= range_2_x
        })
        .count();

    // Your solution here...
    let sol1: u64 = total_completely_overlapping as u64;
    let sol2: u64 = any_overlapping as u64;

    (Solution::from(sol1), Solution::from(sol2))
}
