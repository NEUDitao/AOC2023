use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

/*
*/

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day01").unwrap();

    let new_input = input.split("\n\n");

    let mut calorie_counts = new_input
        .into_iter()
        .map(|single_elf| {
            single_elf
                .split("\n")
                .into_iter()
                .map(|calories| str::parse::<i32>(calories).unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    calorie_counts.sort();
    calorie_counts.reverse();

    // Your solution here...
    let sol1 = *calorie_counts.iter().max().unwrap();
    let sol2 = calorie_counts.into_iter().take(3).sum::<i32>();

    (Solution::from(sol1), Solution::from(sol2))
}
