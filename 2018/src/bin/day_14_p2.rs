fn main() {
    let input: Vec<usize> = aoc::read_input()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();
    let mut scores = vec![3, 7];
    let mut idx_1: usize = 0;
    let mut idx_2: usize = 1;

    loop {
        let new = scores[idx_1] + scores[idx_2];
        if new >= 10 {
            scores.push(new / 10);
        }

        if scores.len() >= input.len() && &scores[(scores.len() - input.len())..] == input {
            break;
        }

        scores.push(new % 10);

        idx_1 = (idx_1 + 1 + scores[idx_1]) % scores.len();
        idx_2 = (idx_2 + 1 + scores[idx_2]) % scores.len();

        if scores.len() >= input.len() && &scores[(scores.len() - input.len())..] == input {
            break;
        }
    }

    let ans = scores.len() - input.len();
    println!("{ans}");
}
