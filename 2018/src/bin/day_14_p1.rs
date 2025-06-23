fn main() {
    let input: usize = aoc::read_input().parse().unwrap();
    let mut scores = vec![3, 7];
    let mut idx_1: usize = 0;
    let mut idx_2: usize = 1;

    while scores.len() < input + 10 {
        let new = scores[idx_1] + scores[idx_2];
        if new >= 10 {
            scores.push(new / 10);
        }
        scores.push(new % 10);

        idx_1 = (idx_1 + 1 + scores[idx_1]) % scores.len();
        idx_2 = (idx_2 + 1 + scores[idx_2]) % scores.len();
    }

    let ans: String = scores[input..(input + 10)]
        .iter()
        .map(|x| x.to_string())
        .collect();
    println!("{ans}");
}
