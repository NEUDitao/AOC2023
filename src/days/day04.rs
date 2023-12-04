use itertools::Itertools;

use crate::{Solution, SolutionPair};
use std::{collections::VecDeque, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

fn get_game_number(row: &str) -> usize {
    row.split_once(": ")
        .unwrap()
        .0
        .split_once(" ")
        .unwrap()
        .1
        .trim()
        .parse::<usize>()
        .unwrap()
}

pub fn how_many_won(row: &str) -> usize {
    let (_, nums) = row.split_once(": ").unwrap();
    let (winners, my_nums) = nums.split_once(" | ").unwrap();
    let winners = winners.split(" ");
    let my_nums = my_nums.split(" ");
    let count = my_nums
        .filter(|num| num != &"" && winners.clone().contains(num))
        .collect::<Vec<_>>();
    count.len()
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day04").unwrap();
    let rows = input.split('\n');

    let first = rows
        .clone()
        .into_iter()
        .map(|row| {
            let count = how_many_won(row);
            if count > 0 {
                2_i32.pow((count - 1) as u32)
            } else {
                0
            }
        })
        .sum::<i32>();

    let mut total_processed = 0;
    let rows = rows.collect::<Vec<_>>();
    let mut won = VecDeque::new();
    for i in 1..rows.len() + 1 {
        won.push_back(i);
    }
    while !won.is_empty() {
        let top = won.pop_front().unwrap();
        let game = rows.get(top - 1).unwrap();
        let count = how_many_won(game);
        let game_number = get_game_number(game);
        for i in 0..count {
            won.push_back(i + game_number + 1)
        }
        total_processed += 1;
    }

    // Your solution here...
    let sol1: u64 = first as u64;
    let sol2: u64 = total_processed as u64;

    (Solution::from(sol1), Solution::from(sol2))
}
