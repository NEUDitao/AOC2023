use itertools::Itertools;

use crate::{Solution, SolutionPair};
use std::{collections::BTreeMap, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input: String = read_to_string("input/day05").unwrap();
    let mut rows = input.split('\n');

    let towers = rows
        .by_ref()
        .take_while(|&line| !line.is_empty())
        .collect::<Vec<_>>();
    let rest = rows.collect::<Vec<_>>();

    // let num_towers: u32 = char::to_digit(
    //     towers
    //         .last()
    //         .unwrap()
    //         .split("   ")
    //         .last()
    //         .unwrap()
    //         .chars()
    //         .next()
    //         .unwrap(),
    //     10,
    // )
    // .unwrap();

    let mut towers = towers.split_last().unwrap().1.to_owned();
    towers.reverse();

    let mut hanoi: BTreeMap<i32, Vec<char>> = BTreeMap::new();

    for row in towers {
        for (idx, mut chunk) in row.chars().chunks(4).into_iter().enumerate() {
            let second_char = chunk.nth(1).unwrap();
            if second_char != ' ' {
                hanoi
                    .entry(i32::try_from(idx).unwrap() + 1)
                    .or_default()
                    .push(second_char);
            }
        }
    }

    // let hanoi = hanoi
    //     .into_iter()
    //     .map(|(i, mut tower)| {
    //         tower.reverse();
    //         (i, tower)
    //     })
    //     .collect::<BTreeMap<_, _>>();

    for instruction in rest {
        let words = instruction.split(' ').collect::<Vec<_>>();
        let num = str::parse::<usize>(words[1]).unwrap();
        let from = str::parse::<i32>(words[3]).unwrap();
        let to = str::parse::<i32>(words[5]).unwrap();

        let tower_from = hanoi.get_mut(&from).unwrap();
        let split_location = tower_from.len() - num;

        let mut takeaway = tower_from.split_off(split_location);
        // takeaway.reverse();

        let tower_to = hanoi.get_mut(&to).unwrap();
        tower_to.append(&mut takeaway);
    }

    println!("{:?}", hanoi);

    for values in hanoi.into_values() {
        println!("{:?}", values.last().unwrap());
    }

    // Your solution here...
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
