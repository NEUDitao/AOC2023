use crate::{Solution, SolutionPair};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::read_to_string,
};

///////////////////////////////////////////////////////////////////////////////

fn calculate_diffs(row: Vec<i64>) -> Vec<i64> {
    let mut res = Vec::new();
    for i in 0..(row.len() - 1) {
        let a = row.get(i).unwrap();
        let b = row.get(i + 1).unwrap();
        res.push(b - a);
    }
    res
}

fn get_extrapolated_value(row: String) -> i64 {
    let mut row = row
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut diffs_list = Vec::new();
    diffs_list.push(row.clone());
    loop {
        let diffs = calculate_diffs(row);
        if diffs.iter().all(|d| d == &0) {
            break;
        }
        diffs_list.push(diffs.clone());
        row = diffs;
    }

    let mut end = 0;
    while let Some(mut diff_list) = diffs_list.pop() {
        end = end + diff_list.pop().unwrap();
    }

    end
}

fn get_extrapolated_value_rev(row: String) -> i64 {
    let mut row = row
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut diffs_list = Vec::new();
    diffs_list.push(row.clone());
    loop {
        let diffs = calculate_diffs(row);
        if diffs.iter().all(|d| d == &0) {
            break;
        }
        diffs_list.push(diffs.clone());
        row = diffs;
    }

    let mut beg = 0;
    while let Some(diff_list) = diffs_list.pop() {
        beg = diff_list.get(0).unwrap() - beg;
    }

    beg
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day09").unwrap();
    let rows = input.split("\n");

    // Your solution here...
    let sol1: i64 = rows
        .clone()
        .map(|r| get_extrapolated_value(r.to_owned()))
        .sum();
    let sol2: i64 = rows.map(|r| get_extrapolated_value_rev(r.to_owned())).sum();

    (Solution::from(sol1), Solution::from(sol2))
}
