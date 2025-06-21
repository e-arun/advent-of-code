use itertools::Itertools;

fn reduce(s: &str) -> usize {
    let mut arr: Vec<_> = s.chars().collect();
    loop {
        let mut flag = true;
        let mut new_arr = Vec::<char>::new();

        let mut i = 0;
        while i < arr.len() {
            if i == arr.len() - 1 {
                new_arr.push(arr[i]);
                i += 1;
                continue;
            }

            if arr[i] != arr[i + 1]
                && arr[i].to_ascii_lowercase() == arr[i + 1].to_ascii_lowercase()
            {
                flag = false;
                i += 2;
                continue;
            }
            new_arr.push(arr[i]);
            i += 1;
            continue;
        }

        if flag {
            break;
        }
        arr = new_arr;
    }

    arr.len()
}

fn main() {
    let input = aoc::read_input();

    let mut ans = input.len();

    for ch in input.chars().map(|x| x.to_ascii_lowercase()).unique() {
        let new_input: String = input
            .chars()
            .filter(|x| x.to_ascii_lowercase() != ch)
            .collect();
        ans = std::cmp::min(ans, reduce(&new_input));
    }
    println!("{ans}");
}
