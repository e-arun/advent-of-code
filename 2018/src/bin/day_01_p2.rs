use std::collections::HashSet;

use aoc::read_lines;

fn main() {
    let mut set = HashSet::<i32>::new();
    let mut cur = 0;
    let arr = read_lines()
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .cycle();

    for val in arr {
        cur += val;
        if set.contains(&cur) {
            println!("{cur}");
            break;
        }
        set.insert(cur);
    }
}
