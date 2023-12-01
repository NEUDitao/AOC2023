use crate::{Solution, SolutionPair};
use maplit::btreemap;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

/*
*/

pub fn solve() -> SolutionPair {
    let lookup = btreemap![
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
    ];

    let input = read_to_string("input/day01").unwrap();

    let new_input = input.split("\n");

    let numbers = new_input
        .into_iter()
        .map(|line| {
            let mut first_digit = None;
            let mut last_digit = None;

            for i in 0..line.len() {
                let substr = line.get(i..line.len()).unwrap();
                let mut set = false;
                for (k, v) in lookup.iter() {
                    if substr.starts_with(k) {
                        if let None = first_digit {
                            first_digit = Some(v.to_owned());
                        }
                        last_digit = Some(v.to_owned());
                        set = true;
                        break;
                    }
                }

                if !set {
                    let c = substr.chars().nth(0).unwrap();
                    let digit = c.to_digit(10);
                    if let Some(digit) = digit {
                        if let None = first_digit {
                            first_digit = Some(digit);
                        }
                        last_digit = Some(digit);
                    }
                }
            }

            let num = format!("{}{}", first_digit.unwrap(), last_digit.unwrap());
            num.parse::<i32>().unwrap()
        })
        .collect::<Vec<i32>>();

    // Your solution here...
    let sol1 = numbers.iter().sum::<i32>();
    // let sol2 = calorie_counts.into_iter().take(3).sum::<i32>();
    let sol2 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
