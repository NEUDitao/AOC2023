use crate::{Solution, SolutionPair};
use std::{
    collections::{BTreeSet, VecDeque},
    fs::read_to_string,
};

///////////////////////////////////////////////////////////////////////////////

pub const START: u32 = 83;
pub const END: u32 = 69;

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day12").unwrap();
    let lines = input.lines();

    let mut map: Vec<Vec<u32>> = Vec::new();
    for row in lines {
        let mut map_row: Vec<u32> = Vec::new();
        for char in row.chars() {
            map_row.push(char as u32);
        }
        map.push(map_row);
    }

    let mut processed: BTreeSet<(i32, i32)> = BTreeSet::new();

    let mut start_loc = None;
    for (i, row) in map.clone().into_iter().enumerate() {
        for (j, _col) in row.into_iter().enumerate() {
            if map[i][j] == START {
                start_loc = Some((i as i32, j as i32));
            }
        }
    }

    let start_loc = start_loc.unwrap();

    let mut queue = VecDeque::from_iter(vec![(start_loc, 0)]);
    processed.insert(start_loc);

    for (i, row) in map.clone().into_iter().enumerate() {
        for (j, col) in row.into_iter().enumerate() {
            if col == ('a' as u32) {
                queue.push_back(((i as i32, j as i32), 0));
                processed.insert((i as i32, j as i32));
            }
        }
    }

    while let Some(((cur_x, cur_y), cnt)) = queue.pop_front() {
        // dbg!((cur_x, cur_y));
        let mut cur_value = map[cur_x as usize][cur_y as usize];
        if cur_value == START {
            cur_value = ('a' as u32) - 1;
        }

        for (next_x, next_y) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next_value = map
                .get((next_x + cur_x) as usize)
                .and_then(|nxt| nxt.get((next_y + cur_y) as usize));

            if let Some(&next_value) = next_value {
                if !processed.contains(&(next_x + cur_x, next_y + cur_y))
                    && cur_value >= (next_value - 1)
                {
                    queue.push_back(((next_x + cur_x, next_y + cur_y), cnt + 1));
                    processed.insert((cur_x + next_x, cur_y + next_y));
                }
                if next_value == END && cur_value == 'z' as u32 {
                    dbg!(cnt + 1);
                    break;
                }
            }
        }
    }

    // Your solution here...
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
