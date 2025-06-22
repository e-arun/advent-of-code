use std::collections::HashMap;

fn main() {
    let lines = aoc::read_lines();

    let initial_state = &lines[0][15..];
    let map: HashMap<&str, &str> = lines[2..]
        .iter()
        .map(|x| x.split(" => ").collect::<Vec<_>>())
        .map(|x| (x[0], x[1]))
        .collect();

    let offset = 500;
    let mut state = ['.'; 1000];
    for (i, char) in initial_state.chars().enumerate() {
        state[i + offset] = char;
    }

    for _ in 0..20 {
        let mut new_state = ['.'; 1000];

        for i in 0..(state.len() - 5) {
            let key: String = state[i..i + 5].iter().collect();
            if let Some(&"#") = map.get(&key[..]) {
                new_state[i + 2] = '#';
            }
        }
        state = new_state;
    }

    let mut ans = 0;
    for (i, char) in state.iter().enumerate() {
        if *char == '#' {
            ans += i - offset;
        }
    }

    println!("{ans}");
}
