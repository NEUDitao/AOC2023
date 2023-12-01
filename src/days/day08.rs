use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

fn get_if_visible(map: &Vec<Vec<u32>>, row: usize, col: usize) -> bool {
    let cur_val = map[row][col];

    row == 0
        || col == 0
        || row == map.len() - 1
        || col == map[row as usize].len() - 1
        || (cur_val > map[row][col - 1] && get_if_visible_left(map, row, col - 1, cur_val))
        || (cur_val > map[row][col + 1] && get_if_visible_right(map, row, col + 1, cur_val))
        || (cur_val > map[row + 1][col] && get_if_visible_down(map, row + 1, col, cur_val))
        || (cur_val > map[row - 1][col] && get_if_visible_up(map, row - 1, col, cur_val))
}

fn get_if_visible_left(map: &Vec<Vec<u32>>, row: usize, col: usize, cur_val: u32) -> bool {
    col == 0 || (cur_val > map[row][col - 1] && get_if_visible_left(map, row, col - 1, cur_val))
}

fn get_if_visible_right(map: &Vec<Vec<u32>>, row: usize, col: usize, cur_val: u32) -> bool {
    col == map.len() - 1
        || (cur_val > map[row][col + 1] && get_if_visible_right(map, row, col + 1, cur_val))
}

fn get_if_visible_up(map: &Vec<Vec<u32>>, row: usize, col: usize, cur_val: u32) -> bool {
    row == 0 || (cur_val > map[row - 1][col] && get_if_visible_up(map, row - 1, col, cur_val))
}

fn get_if_visible_down(map: &Vec<Vec<u32>>, row: usize, col: usize, cur_val: u32) -> bool {
    row == map.len() - 1
        || (cur_val > map[row + 1][col] && get_if_visible_down(map, row + 1, col, cur_val))
}

fn get_val(map: &Vec<Vec<u32>>, row: usize, col: usize) -> i32 {
    let cur_val = map[row][col];
    // let mut tot = 1;

    if col == 0 || col == map.len() - 1 || row == map.len() - 1 || row == 0 {
        0
    } else {
        get_val_left(map, row, col - 1, cur_val)
            * get_val_right(map, row, col + 1, cur_val)
            * get_val_down(map, row + 1, col, cur_val)
            * get_val_up(map, row - 1, col, cur_val)
    }
}

fn get_val_left(map: &Vec<Vec<u32>>, row: usize, col: usize, cur_val: u32) -> i32 {
    if col == 0 {
        1
    } else if cur_val <= map[row][col] {
        1
    } else {
        1 + get_val_left(map, row, col - 1, cur_val)
    }
}

fn get_val_right(map: &Vec<Vec<u32>>, row: usize, col: usize, cur_val: u32) -> i32 {
    if col == map.len() - 1 {
        1
    } else if cur_val <= map[row][col] {
        1
    } else {
        1 + get_val_right(map, row, col + 1, cur_val)
    }
}

fn get_val_up(map: &Vec<Vec<u32>>, row: usize, col: usize, cur_val: u32) -> i32 {
    if row == 0 {
        1
    } else if cur_val <= map[row][col] {
        1
    } else {
        1 + get_val_up(map, row - 1, col, cur_val)
    }
}

fn get_val_down(map: &Vec<Vec<u32>>, row: usize, col: usize, cur_val: u32) -> i32 {
    if row == map.len() - 1 {
        1
    } else if cur_val <= map[row][col] {
        1
    } else {
        1 + get_val_down(map, row + 1, col, cur_val)
    }
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day08").unwrap();
    let rows = input.split('\n');

    let mut map: Vec<Vec<u32>> = Vec::new();
    for row in rows {
        let mut map_row: Vec<u32> = Vec::new();
        for char in row.chars() {
            map_row.push(char.to_digit(10).unwrap());
        }
        map.push(map_row);
    }

    let mut count = 0;
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            let is_visible = get_if_visible(&map, row, col);
            if is_visible {
                count += 1;
            }
        }
    }

    let mut max = 0;

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            let val = get_val(&map, row, col);
            if val > max {
                max = val;
            }
        }
    }

    // dbg!(get_val(&map, 3, 2));
    // dbg!(get_val_left(&map, 3, 2, 3));
    // dbg!(get_val_right(&map, 3, 2, 3));
    // dbg!(get_val_up(&map, 3, 2, 3));
    // dbg!(get_val_down(&map, 3, 2, 3));

    // Your solution here...
    let sol1: u64 = count;
    let sol2 = max;

    (Solution::from(sol1), Solution::from(sol2))
}
