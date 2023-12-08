use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn go_dir(row: String, dir: &Direction) -> String {
    let row = row.split_whitespace().collect::<Vec<_>>();
    let left = &row.get(2).unwrap()[1..4];
    let right = &row.get(3).unwrap()[0..3];
    match dir {
        Direction::Left => left,
        Direction::Right => right,
    }
    .to_owned()
}

fn get_row(rows: &Vec<String>, row_to_find: &str) -> String {
    rows.iter()
        .find(|row| row.starts_with(row_to_find))
        .unwrap()
        .to_owned()
}

fn get_loc(row: &str) -> String {
    row.split_whitespace().next().unwrap().to_owned()
}

fn lcm(a: u128, b: u128) -> u128 {
    a * b / gcd(a, b)
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day08").unwrap();
    let mut rows = input.split('\n');

    let steps = rows
        .next()
        .unwrap()
        .chars()
        .map(|c| {
            if c == 'R' {
                Direction::Right
            } else {
                Direction::Left
            }
        })
        .collect::<Vec<_>>();
    rows.next();
    let rows = rows.map(|s| s.to_owned()).collect::<Vec<_>>();

    let mut current_loc = "AAA".to_owned();
    let mut count = 0;
    let mut index_in_steps = 0;

    loop {
        if index_in_steps >= steps.len() {
            index_in_steps = 0;
        }
        let curr_dir = steps.get(index_in_steps).unwrap();
        let curr_row = get_row(&rows, &current_loc);
        let loc = go_dir(curr_row, curr_dir);

        current_loc = loc;
        index_in_steps += 1;

        count += 1;

        if current_loc == "ZZZ" {
            break;
        }
    }

    // // Your solution here...
    let sol1: u64 = count;

    let mut current_locs = rows
        .clone()
        .into_iter()
        .filter_map(|row| {
            let loc = get_loc(&row);
            if loc.ends_with("A") {
                Some(loc.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let mut count = 0;
    let mut index_in_steps = 0;
    let mut steps_required = Vec::new();

    loop {
        if index_in_steps >= steps.len() {
            index_in_steps = 0;
        }
        let curr_dir = steps.get(index_in_steps).unwrap();

        current_locs = current_locs
            .iter()
            .map(|loc| {
                let curr_row = get_row(&rows, &loc);
                go_dir(curr_row, curr_dir)
            })
            .collect::<Vec<_>>();

        index_in_steps += 1;
        count += 1;

        for loc in &current_locs {
            if loc.ends_with("Z") {
                steps_required.push(count);
            }
        }
        current_locs = current_locs
            .iter()
            .filter(|loc| !loc.ends_with("Z"))
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();

        if count % 100000 == 0 {
            println!("here!");
        }

        if current_locs.is_empty() {
            break;
        }
    }

    let sol2 = steps_required.iter().fold(1, |acc, x| lcm(acc, *x));

    (Solution::from(sol1), Solution::from(sol2))
}
