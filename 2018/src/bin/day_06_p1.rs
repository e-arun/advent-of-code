use std::collections::{HashMap, HashSet};

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

    let mut map = HashMap::<(i32, i32), usize>::new();
    let mut inf_points_idx = HashSet::<usize>::new();
    let mut ctr = HashMap::<usize, i32>::new();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let dist_vec: Vec<_> = points
                .iter()
                .map(|p| dist(p, &(x, y)))
                .enumerate()
                .collect();
            let min_dist = dist_vec.iter().map(|x| x.1).min().unwrap();
            let dist_vec: Vec<_> = dist_vec.into_iter().filter(|x| x.1 == min_dist).collect();
            if dist_vec.len() == 1 {
                map.insert((x, y), dist_vec[0].0);
                let ct = ctr.entry(dist_vec[0].0).or_insert(0);
                *ct += 1;
            }

            if x == min_x || x == max_x || y == min_y || y == max_y {
                inf_points_idx.insert(dist_vec[0].0);
            }
        }
    }

    let ans = ctr
        .iter()
        .filter(|x| !inf_points_idx.contains(x.0))
        .map(|x| x.1)
        .max()
        .unwrap();
    println!("{ans}");
}
