use itertools::{ Itertools};

use crate::{Solution, SolutionPair};
use std::{collections::HashMap,  fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug, Clone)]
struct Monkey<'a> {
    items: Vec<u64>,
    op: [&'a str; 3],
    div_by: u64,
    t: u64,
    f: u64,
}

impl Monkey<'_> {
    fn apply_op(&self, val: u64) -> u64 {
        match self.op[1] {
            "+" => get_val(self.op[0], val) + get_val(self.op[2], val),
            "*" => get_val(self.op[0], val) * get_val(self.op[2], val),
            _ => panic!("uh oh!"),
        }
    }
}

fn get_val(arg: &str, val: u64) -> u64 {
    if arg == "old" {
        val
    } else {
        // println!("uh oh {arg}");
        str::parse::<u64>(arg).unwrap()
    }
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day11").unwrap();
    let lines = input.lines();

    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut cur_monkey = Monkey::default();

    for line in lines {
        let split: Vec<&str> = line.split_ascii_whitespace().collect();
        match split.as_slice() {
            ["Monkey", _] => {
                monkeys.push(cur_monkey);
                cur_monkey = Monkey::default();
            }
            ["Starting", "items:", items @ ..] => {
                let items = items
                    .into_iter()
                    .map(|item| {
                        let no_comma = item.replace(',', "");
                        str::parse::<u64>(&no_comma).unwrap()
                    })
                    .collect::<Vec<_>>();
                cur_monkey.items = items;
            }
            ["Operation:", "new", "=", item_1, op, item_2] => {
                cur_monkey.op = [item_1.to_owned(), op.to_owned(), item_2.to_owned()]
            }
            ["Test:", "divisible", "by", num] => {
                cur_monkey.div_by = str::parse::<u64>(num).unwrap()
            }
            ["If", "true:", "throw", "to", "monkey", n] => {
                cur_monkey.t = str::parse::<u64>(n).unwrap()
            }
            ["If", "false:", "throw", "to", "monkey", n] => {
                cur_monkey.f = str::parse::<u64>(n).unwrap()
            }
            _ => {}
        }
    }

    monkeys.remove(0);
    monkeys.push(cur_monkey);

    let mag_num: u64 = monkeys.iter().map(|monkey| monkey.div_by).product();

    let mut monkey_counts: HashMap<u64, u64> = HashMap::new();

    for _ in 0..10000 {
        // do the rounds
        let mut cur_monkeys = monkeys.clone();
        let next_monkeys = monkeys.clone();

        let mut next_monkeys = next_monkeys
            .into_iter()
            .map(|next_monkey| Monkey {
                items: Vec::new(),
                ..next_monkey
            })
            .collect::<Vec<_>>();

        for i in 0..monkeys.len() {
            let cur_monkey = cur_monkeys[i].clone();

            for item in cur_monkey.items.as_slice() {
                *monkey_counts.entry(i as u64).or_default() += 1;
                let new_val = cur_monkey.apply_op(*item);
                let new_val_2 = new_val % mag_num;
                let tester = new_val % cur_monkey.div_by;
                if tester == 0 {
                    if cur_monkey.t < i as u64 {
                        next_monkeys[cur_monkey.t as usize].items.push(new_val_2);
                    } else {
                        cur_monkeys[cur_monkey.t as usize].items.push(new_val_2);
                    }
                } else {
                    if cur_monkey.f < i as u64 {
                        next_monkeys[cur_monkey.f as usize].items.push(new_val_2);
                    } else {
                        cur_monkeys[cur_monkey.f as usize].items.push(new_val_2);
                    }
                }
            }
        }

        monkeys = next_monkeys;
    }

    let mut sorted_counts = dbg!(monkey_counts).into_values().sorted().rev();
    let count_1 = sorted_counts.next().unwrap();
    let count_2 = sorted_counts.next().unwrap();

    // Your solution here...
    let sol1 = count_1 * count_2;
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
