use itertools::{EitherOrBoth, Itertools};
use serde_json::Value;

use crate::{Solution, SolutionPair};
use std::{cmp::Ordering, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

fn is_in_order(left: &str, right: &str) -> bool {
    let left: Value = serde_json::from_str(left).unwrap();
    let right: Value = serde_json::from_str(right).unwrap();

    compare_order(left, right).unwrap_or_default()
}

fn compare_order(left: Value, right: Value) -> Option<bool> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => {
            let left = left.as_i64().unwrap();
            let right = right.as_i64().unwrap();
            if left == right {
                None
            } else {
                Some(left < right)
            }
        }
        (Value::Array(left), Value::Number(right)) => {
            compare_order(Value::Array(left), Value::Array(vec![Value::Number(right)]))
        }
        (Value::Number(left), Value::Array(right)) => {
            compare_order(Value::Array(vec![Value::Number(left)]), Value::Array(right))
        }
        (Value::Array(left), Value::Array(right)) => {
            // std::iter::zip(left, right).any(|(left, right)| compare_order(left, right))

            let iter = left.into_iter().zip_longest(right);

            for i in iter {
                match i {
                    EitherOrBoth::Both(left, right) => {
                        if let Some(ans) = compare_order(left, right) {
                            return Some(ans);
                        }
                    }
                    EitherOrBoth::Left(_) => return Some(false),
                    EitherOrBoth::Right(_) => return Some(true),
                }
            }
            None
        }
        _ => panic!("types!"),
    }
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day13").unwrap();

    let segmented = input.split("\n\n");

    let pairs = segmented
        .into_iter()
        .map(|segment| {
            let segments = Vec::from_iter(segment.split('\n'));
            (segments[0], segments[1])
        })
        .collect::<Vec<_>>();

    let ans_1: i32 = pairs
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, (left, right))| {
            if is_in_order(left, right) {
                i as i32 + 1
            } else {
                0
            }
        })
        .sum();

    let mut all_packets = pairs
        .into_iter()
        .flat_map(|(l, r)| vec![l, r])
        .collect::<Vec<_>>();

    all_packets.push("[[2]]");
    all_packets.push("[[6]]");

    all_packets.sort_by(|l, r| {
        if is_in_order(l, r) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let i1 = all_packets.iter().position(|s| *s == "[[2]]").unwrap() + 1;
    let i2 = all_packets.iter().position(|s| *s == "[[6]]").unwrap() + 1;

    // Your solution here...
    let sol1 = ans_1;
    let sol2 = i1 * i2;

    (Solution::from(sol1), Solution::from(sol2))
}
