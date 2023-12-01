use crate::{Solution, SolutionPair};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::read_to_string,
};

///////////////////////////////////////////////////////////////////////////////

fn figure_dir(dir: &str) -> (i32, i32) {
    match dir {
        "R" => (1, 0),
        "U" => (0, 1),
        "L" => (-1, 0),
        "D" => (0, -1),
        _ => panic!("fuck"),
    }
}

fn update(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    match (head.0 - tail.0, head.1 - tail.1) {
        (2, 0) => (tail.0 + 1, tail.1),
        (-2, 0) => (tail.0 - 1, tail.1),
        (0, 2) => (tail.0, tail.1 + 1),
        (0, -2) => (tail.0, tail.1 - 1),
        (2, 1) | (1, 2) | (2, 2) => (tail.0 + 1, tail.1 + 1),
        (2, -1) | (1, -2) | (2, -2) => (tail.0 + 1, tail.1 - 1),
        (-2, 1) | (-1, 2) | (-2, 2) => (tail.0 - 1, tail.1 + 1),
        (-2, -1) | (-1, -2) | (-2, -2) => (tail.0 - 1, tail.1 - 1),
        _ => tail,
    }
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day09").unwrap();

    let mut rope_parts: BTreeMap<i32, (i32, i32)> = BTreeMap::new();

    for i in 0..10 {
        rope_parts.insert(i, (0, 0));
    }

    let mut seen: BTreeSet<(i32, i32)> = BTreeSet::new();

    for line in input.lines() {
        let mut line_as_iter = line.split_ascii_whitespace();
        let dir = figure_dir(line_as_iter.next().unwrap());
        let num = str::parse::<i32>(line_as_iter.next().unwrap()).unwrap();

        for _n in 0..num {
            let first_head = rope_parts[&0];
            rope_parts.insert(0, (first_head.0 + dir.0, first_head.1 + dir.1));

            for rope_section in 1..10 {
                let head = rope_parts[&(rope_section - 1)];
                let tail = rope_parts[&rope_section];
                let new_tail = update(head, tail);
                rope_parts.insert(rope_section, new_tail);

                if rope_section == 9 {
                    seen.insert(tail);
                }
            }
        }
    }

    // Your solution here...
    let sol1 = seen.len();
    let sol2 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
