use itertools::join;

use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day10").unwrap();

    let mut cycle = 0;

    let mut reg = 1;
    let mut tot = 0;
    let importants = vec![20, 60, 100, 140, 180, 220];

    let mut tape = Vec::new();
    let mut tapes = Vec::new();

    for line in input.lines() {
        let split: Vec<&str> = line.split_ascii_whitespace().collect();
        let mut loc = cycle % 40;
        match split.as_slice() {
            ["noop"] => {
                if [reg - 1, reg, reg + 1].contains(&loc) {
                    tape.push("#")
                } else {
                    tape.push(".")
                }

                cycle += 1;
                if importants.contains(&cycle) {
                    tot += cycle * reg;
                }
                if tape.len() == 40 {
                    tapes.push(tape);
                    tape = Vec::new();
                }
            }
            ["addx", val] => {
                cycle += 1;
                if [reg - 1, reg, reg + 1].contains(&loc) {
                    tape.push("#")
                } else {
                    tape.push(".")
                }

                if importants.contains(&cycle) {
                    tot += cycle * reg;
                }
                if tape.len() == 40 {
                    tapes.push(tape);
                    tape = Vec::new();
                }

                loc += 1;
                if loc >= 40 {
                    loc = 0;
                }

                cycle += 1;
                if [reg - 1, reg, reg + 1].contains(&loc) {
                    tape.push("#")
                } else {
                    tape.push(".")
                }
                if importants.contains(&cycle) {
                    tot += cycle * reg;
                }
                if tape.len() == 40 {
                    tapes.push(tape);
                    tape = Vec::new();
                }
                reg += str::parse::<i32>(val).unwrap();
            }
            _ => {
                panic!("not a real op???")
            }
        }
    }

    // Your solution here...
    let sol1 = tot;
    let sol2 = 0;
    for tape in tapes {
        let row = join(tape, "");
        println!("{row}");
    }

    (Solution::from(sol1), Solution::from(sol2))
}
