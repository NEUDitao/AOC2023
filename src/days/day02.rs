use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

/*
Day 2 was pretty fun to overengineer! Maybe it's because I was originally
going to do this in Haskell, but I'm still on a bit of a pattern-matching
kick, and impls are fun to create. Really, I could've just done these
all manually with string matching and stuff, but turning them into enums
at least made it so I wouldn't have to juggle around ABC and XYZ.

That's about it. I could make it possibly "cleaner" by making the same
code non-duplicated, and then passing in some form of closure that calculates
the strategy, but whatever, most of the AOC's can be boiled down to that, right?

;)
*/

#[derive(Clone, Copy)]
enum Chosen {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone, Copy)]
enum State {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl From<String> for State {
    fn from(from_str: String) -> Self {
        match from_str.as_ref() {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("invalid!"),
        }
    }
}

impl From<String> for Chosen {
    fn from(from_str: String) -> Self {
        match from_str.as_ref() {
            "X" | "A" => Self::Rock,
            "Y" | "B" => Self::Paper,
            "Z" | "C" => Self::Scissors,
            _ => panic!("invalid!"),
        }
    }
}

impl Chosen {
    fn winner_pts(&self, opponent: Chosen) -> State {
        match (self, opponent) {
            (Self::Rock, Self::Rock)
            | (Self::Paper, Self::Paper)
            | (Self::Scissors, Self::Scissors) => State::Draw,
            (Self::Rock, Self::Paper)
            | (Self::Paper, Self::Scissors)
            | (Self::Scissors, Self::Rock) => State::Loss,
            (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper)
            | (Self::Rock, Self::Scissors) => State::Win,
        }
    }

    fn get_loser(opponent: Chosen) -> Chosen {
        match opponent {
            Self::Rock => Self::Scissors,
            Self::Scissors => Self::Paper,
            Self::Paper => Self::Rock,
        }
    }

    fn get_winner(opponent: Chosen) -> Chosen {
        match opponent {
            Self::Scissors => Self::Rock,
            Self::Paper => Self::Scissors,
            Self::Rock => Self::Paper,
        }
    }
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day02").unwrap();

    let rows = input.split("\n");
    let total_points_from_strat_guide_xyz_is_rps = rows
        .clone()
        .into_iter()
        .map(|row| {
            let [a, b]: [&str] = row.split(" ").collect::<Vec<_>>()[..] else {
            panic!("input is malformed!");
        };
            let opponent = Chosen::from(a.to_owned());
            let my_choice = Chosen::from(b.to_owned());

            my_choice as i32 + my_choice.winner_pts(opponent) as i32
        })
        .sum::<i32>();

    let total_points_from_actual_strat_guide = rows
        .clone()
        .into_iter()
        .map(|row| {
            let [a, b]: [&str] = row.split(" ").collect::<Vec<_>>()[..] else {
            panic!("input is malformed!");
        };
            let opponent = Chosen::from(a.to_owned());
            let strat = State::from(b.to_owned());

            strat as i32
                + match strat {
                    State::Loss => Chosen::get_loser(opponent) as i32,
                    State::Draw => opponent as i32,
                    State::Win => Chosen::get_winner(opponent) as i32,
                }
        })
        .sum::<i32>();

    (
        Solution::from(total_points_from_strat_guide_xyz_is_rps),
        Solution::from(total_points_from_actual_strat_guide),
    )
}
