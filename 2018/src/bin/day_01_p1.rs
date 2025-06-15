use aoc::read_lines;

fn main() {
    let ans: i32 = read_lines()
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .sum();
    println!("{ans}");
}
