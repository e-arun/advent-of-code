use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn turn_left(dir: Dir) -> Dir {
    match dir {
        Dir::Up => Dir::Left,
        Dir::Left => Dir::Down,
        Dir::Down => Dir::Right,
        Dir::Right => Dir::Up,
    }
}

fn turn_right(dir: Dir) -> Dir {
    match dir {
        Dir::Up => Dir::Right,
        Dir::Right => Dir::Down,
        Dir::Down => Dir::Left,
        Dir::Left => Dir::Up,
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim_end().to_owned();

    let lines: Vec<Vec<char>> = input
        .split("\n")
        .map(|x| x.to_string())
        .into_iter()
        .map(|x| x.chars().collect())
        .collect();
    let rows = lines.len();
    let cols = lines.iter().map(|x| x.len()).max().unwrap();

    let mut mat = vec![vec![' '; cols]; rows];
    let mut map = HashMap::<(usize, usize), Dir>::new();
    let mut turns = HashMap::<(usize, usize), i32>::new();

    for r in 0..rows {
        for c in 0..cols {
            if let Some(x) = lines[r].get(c) {
                match *x {
                    '>' => {
                        map.insert((r, c), Dir::Right);
                        mat[r][c] = '-';
                    }
                    '<' => {
                        map.insert((r, c), Dir::Left);
                        mat[r][c] = '-';
                    }
                    '^' => {
                        map.insert((r, c), Dir::Up);
                        mat[r][c] = '|';
                    }
                    'v' => {
                        map.insert((r, c), Dir::Down);
                        mat[r][c] = '|';
                    }
                    _ => {
                        mat[r][c] = *x;
                    }
                }
            }
        }
    }

    for (k, _) in map.iter() {
        turns.insert(*k, 0);
    }

    loop {
        let mut skip_set = HashSet::<(usize, usize)>::new();

        for r in 0..rows {
            for c in 0..cols {
                if !map.contains_key(&(r, c)) {
                    continue;
                }
                if skip_set.contains(&(r, c)) {
                    continue;
                }

                let mut dir = map.remove(&(r, c)).unwrap();
                let mut turn_ct = turns.remove(&(r, c)).unwrap();

                let (nr, nc) = match dir {
                    Dir::Up => (r - 1, c),
                    Dir::Down => (r + 1, c),
                    Dir::Left => (r, c - 1),
                    Dir::Right => (r, c + 1),
                };
                skip_set.insert((nr, nc));

                if map.contains_key(&(nr, nc)) {
                    println!("{},{}", nc, nr);
                    return;
                }

                dir = match mat[nr][nc] {
                    '/' => match dir {
                        Dir::Up => Dir::Right,
                        Dir::Left => Dir::Down,
                        Dir::Right => Dir::Up,
                        Dir::Down => Dir::Left,
                    },
                    '\\' => match dir {
                        Dir::Right => Dir::Down,
                        Dir::Up => Dir::Left,
                        Dir::Down => Dir::Right,
                        Dir::Left => Dir::Up,
                    },
                    '+' => {
                        turn_ct += 1;
                        match turn_ct % 3 {
                            1 => turn_left(dir),
                            2 => dir,
                            0 => turn_right(dir),
                            _ => panic!(),
                        }
                    }
                    _ => dir,
                };

                map.insert((nr, nc), dir);
                turns.insert((nr, nc), turn_ct);

                // let mut disp = vec![vec![' '; cols]; rows];
                // for r in 0..rows {
                //     for c in 0..cols {
                //         disp[r][c] = mat[r][c];
                //         if let Some(dir) = map.get(&(r, c)) {
                //             disp[r][c] = match dir {
                //                 Dir::Up => '^',
                //                 Dir::Right => '>',
                //                 Dir::Down => 'v',
                //                 Dir::Left => '<',
                //             }
                //         }
                //     }
                // }
                // for row in disp.iter() {
                //     println!("{}", row.iter().collect::<String>());
                // }
                // println!();
            }
        }
    }
}
