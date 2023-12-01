use crate::{Solution, SolutionPair};
use std::{collections::HashSet, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day06").unwrap();
    let first_row = input.split('\n').next().unwrap();

    let mut x = 0;
    for idx in 14..(first_row.len()) {
        let seq = &first_row[idx - 14..idx];
        let seq_len = seq.len();
        let uniq: HashSet<char> = HashSet::from_iter(seq.chars());
        if uniq.len() == seq_len {
            x = idx;
            break;
        }
    }

    // Your solution here...
    let sol1: usize = x;
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
