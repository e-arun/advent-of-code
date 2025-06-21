fn parse_tuple(s: &str) -> (i32, i32) {
    let arr: Vec<i32> = s.split(", ").map(|x| x.parse().unwrap()).collect();
    (arr[0], arr[1])
}

fn dist(p1: &(i32, i32), p2: &(i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn main() {
    let points: Vec<(i32, i32)> = aoc::read_lines().iter().map(|x| parse_tuple(x)).collect();

    let min_x = points.iter().map(|p| p.0).min().unwrap();
    let min_y = points.iter().map(|p| p.1).min().unwrap();
    let max_x = points.iter().map(|p| p.0).max().unwrap();
    let max_y = points.iter().map(|p| p.1).max().unwrap();

    let mut ans = 0;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let dist_sum: i32 = points.iter().map(|p| dist(p, &(x, y))).sum();
            if dist_sum < 10_000 {
                ans += 1;
            }
        }
    }
    println!("{ans}");
}
