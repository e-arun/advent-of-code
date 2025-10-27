use std::collections::{BinaryHeap, HashSet};

use regex::Regex;

fn main() {
    let lines = aoc::read_lines();

    let re = Regex::new(r"depth: (\d+)").unwrap();
    let caps = re.captures(&lines[0]).unwrap();
    let depth: usize = caps[1].parse().unwrap();

    let re = Regex::new(r"target: (\d+),(\d+)").unwrap();
    let caps = re.captures(&lines[1]).unwrap();
    let tar_x: usize = caps[1].parse().unwrap();
    let tar_y: usize = caps[2].parse().unwrap();

    let mut mat = [[0; 1_000]; 1_000];

    for x in 0..1_000 {
        for y in 0..1_000 {
            let geo;
            if (x, y) == (0, 0) {
                geo = 0;
            } else if (x, y) == (tar_x, tar_y) {
                geo = 0;
            } else if y == 0 {
                geo = x * 16807;
            } else if x == 0 {
                geo = y * 48271;
            } else {
                geo = mat[x - 1][y] * mat[x][y - 1];
            }
            mat[x][y] = (geo + depth) % 20183;
        }
    }

    let mut vis = HashSet::new();
    let mut pending = BinaryHeap::new();
    pending.push((0, ((0, 0), 't')));

    while !pending.is_empty() {
        let (dist, state) = pending.pop().unwrap();
        let (pos, tool) = state;
        let (x, y) = pos;

        if vis.contains(&state) {
            continue;
        }
        vis.insert(state);

        if pos == (tar_x, tar_y) && tool == 't' {
            println!("{}", -dist);
            break;
        }

        let region = mat[x][y] % 3;

        // invalid states
        if region == 0 && tool == 'n' {
            continue;
        }
        if region == 1 && tool == 't' {
            continue;
        }
        if region == 2 && tool == 'c' {
            continue;
        }

        // change gear
        for new_tool in ['n', 't', 'c'] {
            if new_tool == tool {
                continue;
            }
            pending.push((dist - 7, (pos, new_tool)));
        }

        // move
        if x > 0 {
            pending.push((dist - 1, ((x - 1, y), tool)));
        }
        if y > 0 {
            pending.push((dist - 1, ((x, y - 1), tool)));
        }
        pending.push((dist - 1, ((x, y + 1), tool)));
        pending.push((dist - 1, ((x + 1, y), tool)));
    }
}
