use std::collections::BinaryHeap;

use regex::Regex;

#[derive(Debug)]
struct Bot {
    x: i32,
    y: i32,
    z: i32,
    r: i32,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Cube {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    z1: i32,
    z2: i32,
}

fn has_intersection(cube: &Cube, bot: &Bot) -> bool {
    let x_dist = if cube.x1 <= bot.x && bot.x <= cube.x2 {
        0
    } else if bot.x < cube.x1 {
        cube.x1 - bot.x
    } else {
        bot.x - cube.x2
    };
    let y_dist = if cube.y1 <= bot.y && bot.y <= cube.y2 {
        0
    } else if bot.y < cube.y1 {
        cube.y1 - bot.y
    } else {
        bot.y - cube.y2
    };
    let z_dist = if cube.z1 <= bot.z && bot.z <= cube.z2 {
        0
    } else if bot.z < cube.z1 {
        cube.z1 - bot.z
    } else {
        bot.z - cube.z2
    };

    x_dist + y_dist + z_dist <= bot.r
}

fn get_min_dist(cube: &Cube) -> i32 {
    std::cmp::min(cube.x1.abs(), cube.x2.abs())
        + std::cmp::min(cube.y1.abs(), cube.y2.abs())
        + std::cmp::min(cube.z1.abs(), cube.z2.abs())
}

fn get_edge(cube: &Cube) -> i32 {
    cube.x2 - cube.x1 + 1
}

fn count_in_range(cube: &Cube, bots: &Vec<Bot>) -> i32 {
    let mut ans = 0;
    for bot in bots {
        if has_intersection(cube, bot) {
            ans += 1;
        }
    }
    ans
}

// Solution based off of the discussions in AoC 2018 solutions thread on reddit
// https://www.reddit.com/r/adventofcode/comments/a8s17l/comment/ecm0jes/
// Works similar to binary search, but instead of eliminating the search area,
// right away, we prioritize sub-sections of the search area

fn main() {
    let lines = aoc::read_lines();
    let re = Regex::new(r"pos=<([-]?\d+),([-]?\d+),([-]?\d+)>, r=(\d+)").unwrap();

    let mut bots = Vec::new();
    for line in lines {
        let caps = re.captures(&line).unwrap();
        bots.push(Bot {
            x: caps[1].parse().unwrap(),
            y: caps[2].parse().unwrap(),
            z: caps[3].parse().unwrap(),
            r: caps[4].parse().unwrap(),
        });
    }

    let mut pending = BinaryHeap::new();

    // highest value that does not lead to i32 overflow
    // should be enough to cover all bots
    let max_edge = 1 << 28;
    let max_cube = Cube {
        x1: -max_edge,
        x2: max_edge - 1,
        y1: -max_edge,
        y2: max_edge - 1,
        z1: -max_edge,
        z2: max_edge - 1,
    };
    pending.push((
        count_in_range(&max_cube, &bots),
        -get_min_dist(&max_cube),
        -get_edge(&max_cube),
        max_cube,
    ));

    while !pending.is_empty() {
        let (_, dist, edge, cube) = pending.pop().unwrap();

        if edge == -1 {
            println!("{}", -dist);
            break;
        }

        let mx = cube.x1.midpoint(cube.x2);
        let my = cube.y1.midpoint(cube.y2);
        let mz = cube.z1.midpoint(cube.z2);
        let split_cubes = vec![
            Cube {
                x1: cube.x1,
                x2: mx,
                y1: cube.y1,
                y2: my,
                z1: cube.z1,
                z2: mz,
            },
            Cube {
                x1: mx + 1,
                x2: cube.x2,
                y1: cube.y1,
                y2: my,
                z1: cube.z1,
                z2: mz,
            },
            Cube {
                x1: cube.x1,
                x2: mx,
                y1: my + 1,
                y2: cube.y2,
                z1: cube.z1,
                z2: mz,
            },
            Cube {
                x1: mx + 1,
                x2: cube.x2,
                y1: my + 1,
                y2: cube.y2,
                z1: cube.z1,
                z2: mz,
            },
            Cube {
                x1: cube.x1,
                x2: mx,
                y1: cube.y1,
                y2: my,
                z1: mz + 1,
                z2: cube.z2,
            },
            Cube {
                x1: mx + 1,
                x2: cube.x2,
                y1: cube.y1,
                y2: my,
                z1: mz + 1,
                z2: cube.z2,
            },
            Cube {
                x1: cube.x1,
                x2: mx,
                y1: my + 1,
                y2: cube.y2,
                z1: mz + 1,
                z2: cube.z2,
            },
            Cube {
                x1: mx + 1,
                x2: cube.x2,
                y1: my + 1,
                y2: cube.y2,
                z1: mz + 1,
                z2: cube.z2,
            },
        ];
        for new_cube in split_cubes {
            pending.push((
                count_in_range(&new_cube, &bots),
                -get_min_dist(&new_cube),
                -get_edge(&new_cube),
                new_cube,
            ));
        }
    }
}
