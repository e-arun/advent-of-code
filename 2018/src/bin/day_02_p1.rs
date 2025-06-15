use std::collections::HashMap;

use aoc::read_lines;

fn main() {
    let lines = read_lines();
    let mut val2 = 0;
    let mut val3 = 0;

    for line in lines {
        let mut map = HashMap::new();

        for ch in line.chars() {
            let ct = map.entry(ch).or_insert(0);
            *ct += 1;
        }

        if map.values().any(|&x| x == 2) {
            val2 += 1;
        }
        if map.values().any(|&x| x == 3) {
            val3 += 1;
        }
    }

    let ans = val2 * val3;
    println!("{ans}")
}
