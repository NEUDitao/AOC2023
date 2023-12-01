use crate::{Solution, SolutionPair};
use std::collections::HashSet;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input: String = read_to_string("input/day03").unwrap();

    let rows = input.split("\n");
    let total_priorities = rows
        .clone()
        .into_iter()
        .map(|row| {
            let middle = row.chars().count() / 2;
            let (str1, str2) = row.split_at(middle);
            // println!("FUCK {:?} {:?}", str1, str2);
            let str1_chars = str1.chars().collect::<HashSet<_>>();
            let matching_char = str2
                .chars()
                .find(|str2_char| str1_chars.contains(str2_char))
                .expect("malformed input, or you're not splitting in half properly!");
            let mut buffer = [0];
            matching_char.encode_utf8(&mut buffer);
            let char_num = buffer[0];
            let offset = if char_num > 96 { 96 } else { 38 };

            (buffer[0] - offset) as u64
        })
        .sum();

    let vec_rows = Vec::from_iter(rows);
    let chunks = vec_rows.chunks(3);
    let priorities_of_threes = chunks
        .into_iter()
        .map(|chunk| {
            let [r1, r2, r3] = chunk else {
            panic!("malformed input, not multiple of 3!");
          };

            let mut common_letter = '=';
            for c1 in r1.chars() {
                for c2 in r2.chars() {
                    if c1 == c2 {
                        for c3 in r3.chars() {
                            if c3 == c1 {
                                common_letter = c1;
                                break;
                            }
                        }
                    }
                }
            }

            let mut buffer = [0];
            common_letter.encode_utf8(&mut buffer);
            let char_num = buffer[0];
            let offset = if char_num > 96 { 96 } else { 38 };

            (buffer[0] - offset) as u64
        })
        .sum();

    // Your solution here...
    let sol1: u64 = total_priorities;
    let sol2: u64 = priorities_of_threes;

    (Solution::from(sol1), Solution::from(sol2))
}
