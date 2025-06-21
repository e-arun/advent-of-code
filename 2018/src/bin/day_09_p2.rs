use std::collections::{HashMap, LinkedList};

fn main() {
    let input = aoc::read_input();
    let words: Vec<_> = input.split_whitespace().collect();
    let players: i32 = words[0].parse().unwrap();
    let last_marble: i32 = words[6].parse::<i32>().unwrap() * 100;

    let mut arr = LinkedList::from([2, 1, 0]);
    let mut scores = HashMap::<i32, i64>::new();

    for i in 3..=last_marble {
        if i % 23 != 0 {
            let mut tmp = arr.split_off(2);
            tmp.push_front(i);
            tmp.append(&mut arr);
            arr = tmp;
            continue;
        }

        let player_id = (i - 1) % players;
        let ct = scores.entry(player_id).or_insert(0);
        *ct += i as i64;

        let mut tmp = arr.split_off(arr.len() - 7);
        *ct += tmp.pop_front().unwrap() as i64;
        tmp.append(&mut arr);
        arr = tmp;
    }

    let ans = scores.values().max().unwrap();
    println!("{ans}");
}
