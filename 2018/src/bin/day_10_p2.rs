use std::collections::HashSet;

#[derive(Debug)]
struct Point {
    pos: (i32, i32),
    velocity: (i32, i32),
}

fn parse_tuple(s: &str) -> (i32, i32) {
    let arr: Vec<_> = s.split(",").map(|x| x.trim()).collect();
    (arr[0].parse().unwrap(), arr[1].parse().unwrap())
}

fn parse_point(s: &str) -> Point {
    Point {
        pos: parse_tuple(&s[10..24]),
        velocity: parse_tuple(&s[36..42]),
    }
}

fn main() {
    let lines = aoc::read_lines();
    let points: Vec<Point> = lines.iter().map(|x| parse_point(x)).collect();

    for t in 0..100_000 {
        let arr: HashSet<(i32, i32)> = points
            .iter()
            .map(|p| (p.pos.0 + t * p.velocity.0, p.pos.1 + t * p.velocity.1))
            .collect();

        let min_x = arr.iter().map(|x| x.0).min().unwrap();
        let min_y = arr.iter().map(|x| x.1).min().unwrap();
        let max_x = arr.iter().map(|x| x.0).max().unwrap();
        let max_y = arr.iter().map(|x| x.1).max().unwrap();

        if (max_x - min_x) > 100 || (max_y - min_y) > 100 {
            continue;
        }

        println!("t = {}", t);
        for y in min_y..=max_y {
            let mut line = Vec::new();
            for x in min_x..=max_x {
                if arr.contains(&(x, y)) {
                    line.push("#")
                } else {
                    line.push(" ")
                }
            }
            println!("{}", line.join(""));
        }
        println!();
    }
}
