use regex::Regex;
use std::{
    cmp,
    collections::{HashMap, HashSet},
};

#[derive(Debug)]
struct Input {
    first_var: char,
    single: i32,
    start: i32,
    end: i32,
}

#[derive(Debug)]
struct Bounds {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

// fn print_map(map: &HashMap<(i32, i32), char>, bounds: &Bounds) {
//     for y in bounds.min_y..=bounds.max_y {
//         for x in (bounds.min_x - 1)..=(bounds.max_x + 1) {
//             print!("{}", map.get(&(x, y)).unwrap_or(&'.'));
//         }
//         println!();
//     }
// }

fn fill_water(map: &mut HashMap<(i32, i32), char>, bounds: &Bounds) -> i32 {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut pending = Vec::<(i32, i32)>::new();
    pending.push((500, 1));

    let mut filled = true;

    while !pending.is_empty() {
        let pos = pending.pop().unwrap();
        if visited.contains(&pos) {
            continue;
        }
        if pos.1 > bounds.max_y {
            continue;
        }
        visited.insert(pos);

        let down = (pos.0, pos.1 + 1);
        if !map.contains_key(&down) {
            pending.push(down);
            continue;
        }

        let mut left_barrier = false;
        let mut left = (pos.0 - 1, pos.1);

        loop {
            if map.contains_key(&left) {
                left_barrier = true;
                break;
            }
            let left_down = (left.0, left.1 + 1);
            if !map.contains_key(&left_down) {
                break;
            }
            visited.insert(left);
            left = (left.0 - 1, left.1);
        }

        let mut right_barrier = false;
        let mut right = (pos.0 + 1, pos.1);

        loop {
            if map.contains_key(&right) {
                right_barrier = true;
                break;
            }
            let right_down = (right.0, right.1 + 1);
            if !map.contains_key(&right_down) {
                break;
            }
            visited.insert(right);
            right = (right.0 + 1, right.1);
        }

        if left_barrier && right_barrier {
            filled = false;
            for x in (left.0 + 1)..right.0 {
                map.insert((x, pos.1), '~');
            }
        }
        if !left_barrier {
            pending.push(left);
        }
        if !right_barrier {
            pending.push(right);
        }
    }
    if filled {
        for pos in &visited {
            if !map.contains_key(pos) {
                map.insert(*pos, '|');
            }
        }
        let water = map
            .iter()
            .filter(|((_, y), v)| (**v != '#') && (*y >= bounds.min_y))
            .count();
        return water as i32;
    }
    return 0;
}

fn main() {
    let lines = aoc::read_lines();
    let mut inputs = Vec::<Input>::new();

    let re = Regex::new(r"(x|y)=(\d+), (x|y)=(\d+)\.\.(\d+)").unwrap();
    for line in lines {
        let caps = re.captures(&line).unwrap();
        let input = Input {
            first_var: caps[1].chars().next().unwrap(),
            single: caps[2].parse().unwrap(),
            start: caps[4].parse().unwrap(),
            end: caps[5].parse().unwrap(),
        };
        inputs.push(input);
    }

    let mut bounds = Bounds {
        min_x: i32::MAX,
        min_y: i32::MAX,
        max_x: 0,
        max_y: 0,
    };

    for input in &inputs {
        if input.first_var == 'x' {
            bounds.min_x = cmp::min(bounds.min_x, input.single);
            bounds.max_x = cmp::max(bounds.max_x, input.single);

            bounds.min_y = cmp::min(bounds.min_y, input.start);
            bounds.max_y = cmp::max(bounds.max_y, input.end);
        } else {
            bounds.min_y = cmp::min(bounds.min_y, input.single);
            bounds.max_y = cmp::max(bounds.max_y, input.single);

            bounds.min_x = cmp::min(bounds.min_x, input.start);
            bounds.max_x = cmp::max(bounds.max_x, input.end);
        }
    }

    let mut map = HashMap::<(i32, i32), char>::new();
    for input in &inputs {
        if input.first_var == 'x' {
            for y in input.start..=input.end {
                map.insert((input.single, y), '#');
            }
        } else {
            for x in input.start..=input.end {
                map.insert((x, input.single), '#');
            }
        }
    }

    loop {
        let water = fill_water(&mut map, &bounds);
        if water != 0 {
            println!("{}", water);
            break;
        }
    }
}
