use std::collections::{BinaryHeap, HashMap, HashSet};

fn process_inp(inp: &[char], map: &mut HashMap<(i32, i32), HashSet<char>>) {
    let mut pos_arr = Vec::<(i32, i32)>::new();
    let mut cur_pos = (0, 0);

    for dir in inp {
        if !map.contains_key(&cur_pos) {
            map.insert(cur_pos, HashSet::new());
        }

        if *dir == '(' {
            pos_arr.push(cur_pos);
            continue;
        }
        if *dir == ')' {
            cur_pos = pos_arr.pop().unwrap();
            continue;
        }
        if *dir == '|' {
            cur_pos = pos_arr[pos_arr.len() - 1];
            continue;
        }

        let dirs = map.get_mut(&cur_pos).unwrap();
        dirs.insert(*dir);

        match dir {
            'N' => cur_pos = (cur_pos.0, cur_pos.1 - 1),
            'E' => cur_pos = (cur_pos.0 + 1, cur_pos.1),
            'W' => cur_pos = (cur_pos.0 - 1, cur_pos.1),
            'S' => cur_pos = (cur_pos.0, cur_pos.1 + 1),
            _ => panic!("Unexpected dir {}", dir),
        }
    }
}

fn get_ans(map: HashMap<(i32, i32), HashSet<char>>) -> i32 {
    let mut ans = 0;
    let mut vis = HashSet::new();
    let mut pending = BinaryHeap::new();

    pending.push((0, (0, 0)));

    while !pending.is_empty() {
        let (dist, cur_pos) = pending.pop().unwrap();
        if vis.contains(&cur_pos) {
            continue;
        }
        vis.insert(cur_pos);
        if -dist >= 1000 {
            ans += 1;
        }

        if !map.contains_key(&cur_pos) {
            continue;
        }
        let dirs = map.get(&cur_pos).unwrap();
        for dir in dirs {
            match dir {
                'N' => pending.push((dist - 1, (cur_pos.0, cur_pos.1 - 1))),
                'E' => pending.push((dist - 1, (cur_pos.0 + 1, cur_pos.1))),
                'W' => pending.push((dist - 1, (cur_pos.0 - 1, cur_pos.1))),
                'S' => pending.push((dist - 1, (cur_pos.0, cur_pos.1 + 1))),
                _ => panic!("Unexpected dir {}", dir),
            }
        }
    }

    ans
}

fn main() {
    let inp = aoc::read_input();
    let inp: Vec<_> = inp.chars().collect();

    let mut map = HashMap::<(i32, i32), HashSet<char>>::new();
    process_inp(&inp[1..inp.len() - 1], &mut map);
    let ans = get_ans(map);

    println!("{}", ans);
}
