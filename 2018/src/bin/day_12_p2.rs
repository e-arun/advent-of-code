use std::collections::HashMap;

fn get_ans(state: &[char; 10_000]) -> i64 {
    let mut ans = 0 as i64;
    let offset = (state.len() / 2) as i64;

    for (i, char) in state.iter().enumerate() {
        if *char == '#' {
            ans += (i as i64) - offset;
        }
    }
    ans
}

fn main() {
    let lines = aoc::read_lines();

    let initial_state = &lines[0][15..];
    let map: HashMap<&str, &str> = lines[2..]
        .iter()
        .map(|x| x.split(" => ").collect::<Vec<_>>())
        .map(|x| (x[0], x[1]))
        .collect();

    let mut state = ['.'; 10_000];
    let offset = state.len() / 2;
    for (i, char) in initial_state.chars().enumerate() {
        state[i + offset] = char;
    }

    let mut prev_ans = 0;
    let mut diff_arr = Vec::<i64>::new();

    for iter in 0..200 {
        let mut new_state = ['.'; 10_000];

        for i in 0..(state.len() - 5) {
            let key: String = state[i..i + 5].iter().collect();
            if let Some(&"#") = map.get(&key[..]) {
                new_state[i + 2] = '#';
            }
        }
        state = new_state;

        let ans = get_ans(&state);
        let diff = ans - prev_ans;
        prev_ans = ans;

        if diff_arr.len() > 10 && diff_arr[diff_arr.len() - 10..].iter().all(|x| *x == diff) {
            let ans = ans + (50000000000 - iter - 1) * diff;
            println!("{ans}");
            return;
        }
        diff_arr.push(diff);
    }

    println!("Failed to find the answer!");
}
