use std::io;
use std::io::Read;

pub fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input.trim().to_owned()
}

pub fn read_lines() -> Vec<String> {
    read_input().split("\n").map(|x| x.to_string()).collect()
}
