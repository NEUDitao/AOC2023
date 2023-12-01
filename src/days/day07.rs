use crate::{Solution, SolutionPair};
use std::{collections::BTreeMap, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day07").unwrap();
    let mut rows = input.split('\n');

    let mut loc: Vec<&str> = vec!["/"];
    let mut sizes: BTreeMap<String, i32> = BTreeMap::new();

    rows.next();
    for line in rows {
        let split: Vec<&str> = line.split(' ').collect();
        match split.as_slice() {
            // ls
            ["$", _] => {
                // do nothing, since we handle the other cases :D
            }
            // cd
            ["$", "cd", ".."] => {
                loc.pop();
            }
            // cd
            ["$", "cd", c] => {
                loc.push(*c);
            }
            // dir
            ["dir", _] => {
                // skip
            }
            // size
            [size, _] => {
                for idx in 0..loc.len() {
                    let location = &loc[..=idx];
                    let location = location.join("/");
                    *sizes.entry((location).to_owned()).or_default() +=
                        str::parse::<i32>(size).unwrap();
                }
            }
            _ => panic!("uh oh"),
        }
    }

    // Your solution here...
    let sol1: i32 = sizes
        .clone()
        .into_values()
        .filter(|size| size <= &100000)
        .sum();

    let tot_size = *sizes.get("/").unwrap();
    let remaining_size = 70_000_000 - dbg!(tot_size);
    let needed_rem = 30_000_000 - remaining_size;
    let sol2 = sizes
        .into_values()
        .filter(|size| size >= &needed_rem)
        .min()
        .unwrap();

    (Solution::from(sol1), Solution::from(sol2))
}
