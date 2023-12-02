use maplit::btreemap;

use crate::{Solution, SolutionPair};
use std::{collections::BTreeMap, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

/*
*/

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day02").unwrap();

    let rows = input.split("\n");

    let lookup = btreemap! {
        "red" => 12,
        "green" => 13,
        "blue" => 14,
    };

    let sol1 = rows
        .clone()
        .into_iter()
        .map(|row| {
            let (game_and_id, games) = row.split_once(':').unwrap();
            let is_valid = games.trim().split("; ").all(|game| {
                let balls = game.split(", ");
                balls.into_iter().all(|ball| {
                    let (count, color) = ball.split_once(' ').unwrap();
                    count.parse::<i32>().unwrap() <= *lookup.get(color).unwrap()
                })
            });

            if is_valid {
                game_and_id
                    .split_once(' ')
                    .unwrap()
                    .1
                    .parse::<i32>()
                    .unwrap()
            } else {
                0
            }
        })
        .sum::<i32>();

    let sol2 = rows
        .into_iter()
        .map(|row| {
            let mut balls_seen = BTreeMap::new();

            let (_, games) = row.split_once(':').unwrap();
            let counts_and_colors = games
                .trim()
                .split("; ")
                .flat_map(|game| {
                    let balls = game.split(", ");
                    balls.into_iter().map(|ball| ball.split_once(' ').unwrap())
                })
                .collect::<Vec<_>>();

            for (count, color) in counts_and_colors {
                let count = count.parse::<i32>().unwrap();
                balls_seen
                    .entry(color)
                    .and_modify(|old| {
                        if *old < count {
                            *old = count
                        }
                    })
                    .or_insert(count);
            }

            balls_seen.get("red").unwrap()
                * balls_seen.get("blue").unwrap()
                * balls_seen.get("green").unwrap()
        })
        .sum::<i32>();

    (Solution::from(sol1), Solution::from(sol2))
}
