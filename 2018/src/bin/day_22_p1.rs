use regex::Regex;

fn main() {
    let lines = aoc::read_lines();

    let re = Regex::new(r"depth: (\d+)").unwrap();
    let caps = re.captures(&lines[0]).unwrap();
    let depth: usize = caps[1].parse().unwrap();

    let re = Regex::new(r"target: (\d+),(\d+)").unwrap();
    let caps = re.captures(&lines[1]).unwrap();
    let tar_x: usize = caps[1].parse().unwrap();
    let tar_y: usize = caps[2].parse().unwrap();

    let mut mat = [[0; 1_000]; 1_000];
    let mut risk = 0;

    for x in 0..=tar_x {
        for y in 0..=tar_y {
            let geo;
            if (x, y) == (0, 0) {
                geo = 0;
            } else if (x, y) == (tar_x, tar_y) {
                geo = 0;
            } else if y == 0 {
                geo = x * 16807;
            } else if x == 0 {
                geo = y * 48271;
            } else {
                geo = mat[x - 1][y] * mat[x][y - 1];
            }
            mat[x][y] = (geo + depth) % 20183;
            risk += mat[x][y] % 3;
        }
    }

    println!("{}", risk);
}
