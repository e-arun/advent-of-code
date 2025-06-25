use std::collections::{HashMap, HashSet};

fn get_order(mat: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut ans = Vec::new();
    for r in 0..mat.len() {
        for c in 0..mat[0].len() {
            if mat[r][c] == 'E' || mat[r][c] == 'G' {
                ans.push((r, c));
            }
        }
    }
    ans
}

fn get_targets(mat: &Vec<Vec<char>>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let enemy = match mat[pos.0][pos.1] {
        'E' => 'G',
        'G' => 'E',
        _ => panic!(),
    };

    let mut ans = Vec::new();
    for r in 0..mat.len() {
        for c in 0..mat[0].len() {
            if mat[r][c] == enemy {
                ans.push((r, c));
            }
        }
    }
    ans
}

fn get_moves(r: usize, c: usize) -> [(usize, usize); 4] {
    [(r + 1, c), (r - 1, c), (r, c - 1), (r, c + 1)]
}

fn get_open(mat: &Vec<Vec<char>>, targets: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut ans = Vec::new();
    for (r, c) in targets {
        for (nr, nc) in get_moves(*r, *c) {
            if mat[nr][nc] == '.' {
                ans.push((nr, nc));
            }
        }
    }
    ans
}

fn get_nearby_target(
    mat: &Vec<Vec<char>>,
    pos: (usize, usize),
    hp_map: &HashMap<(usize, usize), i32>,
) -> Option<(usize, usize)> {
    let (r, c) = pos;
    let enemy = match mat[r][c] {
        'E' => 'G',
        'G' => 'E',
        _ => panic!(),
    };

    get_moves(r, c)
        .into_iter()
        .filter(|(nr, nc)| mat[*nr][*nc] == enemy)
        .min_by_key(|pos| (hp_map.get(pos).unwrap(), pos.0, pos.1))
}

fn find_closest_open(
    mat: &Vec<Vec<char>>,
    pos: (usize, usize),
    opens: &Vec<(usize, usize)>,
) -> Option<(usize, usize)> {
    let opens: HashSet<_> = opens.iter().collect();
    let mut cur = vec![pos];
    let mut next = Vec::<(usize, usize)>::new();
    let mut candidates = Vec::<(usize, usize)>::new();
    let mut vis = HashSet::<(usize, usize)>::new();

    while cur.len() != 0 {
        for (r, c) in cur {
            vis.insert((r, c));

            for (nr, nc) in get_moves(r, c) {
                if opens.contains(&(nr, nc)) {
                    candidates.push((nr, nc));
                    continue;
                }
                if mat[nr][nc] == '.' && !vis.contains(&(nr, nc)) {
                    next.push((nr, nc));
                    vis.insert((nr, nc));
                }
            }
        }

        cur = next;
        next = Vec::new();

        if candidates.len() != 0 {
            break;
        }
    }

    candidates.sort();
    candidates.get(0).copied()
}

fn find_next_move(
    mat: &Vec<Vec<char>>,
    pos: (usize, usize),
    opens: &Vec<(usize, usize)>,
) -> Option<(usize, usize)> {
    let Some(closest_open) = find_closest_open(mat, pos, opens) else {
        return None;
    };

    let mut cur = vec![closest_open];
    let mut next = Vec::<(usize, usize)>::new();
    let mut candidates = Vec::<(usize, usize)>::new();
    let mut vis = HashSet::<(usize, usize)>::new();

    while cur.len() != 0 {
        for (r, c) in cur {
            vis.insert((r, c));

            for (nr, nc) in get_moves(r, c) {
                if (nr, nc) == pos {
                    candidates.push((r, c));
                    continue;
                }
                if mat[nr][nc] == '.' && !vis.contains(&(nr, nc)) {
                    next.push((nr, nc));
                    vis.insert((nr, nc));
                }
            }
        }

        cur = next;
        next = Vec::new();

        if candidates.len() != 0 {
            break;
        }
    }

    candidates.sort();
    candidates.get(0).copied()
}

fn main() {
    let lines = aoc::read_lines();

    let original_mat: Vec<Vec<char>> = lines.into_iter().map(|x| x.chars().collect()).collect();

    'power: for elf_power in 4..200 {
        let mut mat: Vec<Vec<char>> = original_mat.clone();
        let mut hp_map: HashMap<(usize, usize), i32> =
            get_order(&mat).into_iter().map(|pos| (pos, 200)).collect();
        let mut rounds = 0;

        'main: loop {
            let mut killed = HashSet::<(usize, usize)>::new();

            for pos in get_order(&mat).into_iter() {
                if killed.contains(&pos) {
                    continue;
                }

                let targets = get_targets(&mat, pos);
                if targets.len() == 0 {
                    break 'main;
                }

                let mut final_pos = pos;

                if let None = get_nearby_target(&mat, pos, &hp_map) {
                    let opens = get_open(&mat, &targets);
                    let Some(new_pos) = find_next_move(&mat, pos, &opens) else {
                        continue;
                    };

                    let old_hp = hp_map.remove(&pos).unwrap();
                    hp_map.insert(new_pos, old_hp);
                    final_pos = new_pos;
                    mat[new_pos.0][new_pos.1] = mat[pos.0][pos.1];
                    mat[pos.0][pos.1] = '.';
                }

                let Some(enemy_pos) = get_nearby_target(&mat, final_pos, &hp_map) else {
                    continue;
                };

                let dmg = match mat[final_pos.0][final_pos.1] {
                    'G' => 3,
                    'E' => elf_power,
                    _ => panic!(),
                };
                let enemy_hp = hp_map.entry(enemy_pos).or_insert(0);
                *enemy_hp -= dmg;
                if *enemy_hp <= 0 {
                    if mat[enemy_pos.0][enemy_pos.1] == 'E' {
                        continue 'power;
                    }
                    hp_map.remove(&enemy_pos);
                    mat[enemy_pos.0][enemy_pos.1] = '.';
                    killed.insert(enemy_pos);
                }
            }
            rounds += 1;
        }

        let ans = rounds * hp_map.values().sum::<i32>();
        println!("{ans}");
        break;
    }
}
