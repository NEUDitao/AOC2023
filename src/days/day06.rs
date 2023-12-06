use crate::{Solution, SolutionPair};
use std::{collections::HashSet, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day06").unwrap();
    let mut rows = input.split('\n');
    let mut time = rows.next().unwrap().split_whitespace();
    let mut distance = rows.next().unwrap().split_whitespace();

    time.next();
    distance.next();

    let zip = time.zip(distance);
    let res = zip
        .clone()
        .map(|(time, distance)| {
            let max_time = time.parse::<u64>().unwrap();
            let distance_to_beat = distance.parse::<u64>().unwrap();
            (1..max_time)
                .map(|n| {
                    if n * (max_time - n) > distance_to_beat {
                        1
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .fold(1, |acc, e| acc * e);

    let (time, distance) = zip.fold(
        ("".to_owned(), "".to_owned()),
        |(acc_time, acc_distance), (time, distance)| {
            (
                format!("{}{}", acc_time, time),
                format!("{}{}", acc_distance, distance),
            )
        },
    );
    let max_time = time.parse::<usize>().unwrap();
    let distance_to_beat = distance.parse::<usize>().unwrap();
    let res_2 = (1..max_time)
        .map(|n| {
            if n * (max_time - n) > distance_to_beat {
                1
            } else {
                0
            }
        })
        .sum::<usize>();

    // Your solution here...
    let sol1: usize = res;
    let sol2: usize = res_2;

    (Solution::from(sol1), Solution::from(sol2))
}
