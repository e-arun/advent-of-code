use std::str::FromStr;

use regex::Regex;

#[derive(Debug)]
struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl FromStr for Claim {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        let caps = re.captures(s).unwrap();

        let claim = Claim {
            id: caps[1].parse().unwrap(),
            left: caps[2].parse().unwrap(),
            top: caps[3].parse().unwrap(),
            width: caps[4].parse().unwrap(),
            height: caps[5].parse().unwrap(),
        };
        Ok(claim)
    }
}

fn main() {
    let lines = aoc::read_lines();
    let claims: Vec<Claim> = lines.into_iter().map(|x| x.parse().unwrap()).collect();

    let mut mat = [[0; 1200]; 1200];
    for claim in claims.iter() {
        for x in claim.left..(claim.left + claim.width) {
            for y in claim.top..(claim.top + claim.height) {
                mat[x][y] += 1;
            }
        }
    }

    'mainLoop: for claim in claims.iter() {
        for x in claim.left..(claim.left + claim.width) {
            for y in claim.top..(claim.top + claim.height) {
                if mat[x][y] != 1 {
                    continue 'mainLoop;
                }
            }
        }
        println!("{}", claim.id)
    }
}
