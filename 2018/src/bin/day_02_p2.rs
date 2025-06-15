use std::collections::HashSet;

use aoc::read_lines;

fn update_string(x: &str, i: usize, ch: char) -> String {
    let mut arr: Vec<_> = x.chars().collect();
    arr[i] = ch;
    arr.into_iter().collect()
}
fn remove_chr(x: &str, i: usize) -> String {
    let mut arr: Vec<_> = x.chars().collect();
    arr.remove(i);
    arr.into_iter().collect()
}

fn main() {
    let lines = read_lines();
    let mut set = HashSet::new();

    for line in lines {
        for (i, ch) in line.chars().enumerate() {
            for nch in 'a'..='z' {
                if ch == nch {
                    continue;
                }
                let new_str = update_string(&line, i, nch);
                if set.contains(&new_str) {
                    let common = remove_chr(&line, i);
                    println!("{common}");
                    return;
                }
            }
        }
        set.insert(line);
    }
}
