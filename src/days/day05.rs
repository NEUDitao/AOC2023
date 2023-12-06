use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

// fn trace_through_maps(init_seed: u64, maps: &Vec<BTreeMap<u64, u64>>) -> u64 {
//     let mut seed_loc = init_seed;
//     for map in maps {
//         let maybe_new_seed_loc = map.get(&seed_loc);
//         if let Some(new_seed_loc) = maybe_new_seed_loc {
//             seed_loc = *new_seed_loc;
//         }
//     }
//     seed_loc
// }

fn trace_through_maps(init_seed: u64, maps: &Vec<Vec<(u64, u64, u64)>>) -> u64 {
    let mut curr_seed = init_seed;
    for row_map in maps {
        for row in row_map {
            let (dest_start, source_start, range) = row;
            if source_start <= &curr_seed && curr_seed < source_start + range {
                let diff = curr_seed - source_start;
                curr_seed = dest_start + diff;
                break;
            }
        }
    }
    curr_seed
}

pub fn solve() -> SolutionPair {
    let input: String = read_to_string("input/day05").unwrap();
    let mut rows = input.split('\n');

    let seeds = rows.next().unwrap().split_whitespace();
    let seeds = seeds
        .into_iter()
        .filter_map(|seed| {
            if seed == "seeds:" {
                None
            } else {
                Some(seed.parse::<u64>().unwrap())
            }
        })
        .collect::<Vec<_>>();

    let mut maps = Vec::new();
    rows.next();

    for _ in 0..7 {
        // x to y map
        rows.next();

        let mut row_maps = Vec::new();

        loop {
            let row = rows.next();
            if let Some(row) = row {
                if row == "" {
                    break;
                }
                let mut row = row.split_whitespace().map(|n| n.parse::<u64>().unwrap());
                row_maps.push((
                    row.next().unwrap(),
                    row.next().unwrap(),
                    row.next().unwrap(),
                ));
            } else {
                break;
            }
        }

        maps.push(row_maps);
        // let map = solve_shit(row_maps);
        // maps.push(map);
    }
    let sol1 = seeds
        .clone()
        .into_iter()
        .map(|seed| trace_through_maps(seed, &maps))
        .min()
        .unwrap();

    let mut min_seen_so_far: u64 = u64::MAX;

    for seed in seeds.chunks(2) {
        println!("one iter");
        for s in seed[0]..seed[0] + seed[1] {
            let ans = trace_through_maps(s, &maps);
            if ans < min_seen_so_far {
                min_seen_so_far = ans;
            }
        }
    }
    let sol2 = min_seen_so_far;

    // let seeds = seeds
    //     .chunks(2)
    //     .flat_map(|chunk| (chunk[0]..chunk[0] + chunk[1]))
    //     .collect::<Vec<u64>>();

    // Your solution here...
    // let sol2: u64 = seeds
    //     .into_iter()
    //     .map(|seed| trace_through_maps(seed, &maps))
    //     .min()
    //     .unwrap();

    (Solution::from(sol1), Solution::from(sol2))
}
