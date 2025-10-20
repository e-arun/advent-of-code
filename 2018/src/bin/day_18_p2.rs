use std::collections::HashMap;

fn count_adj(mat: &Vec<Vec<char>>, r: usize, c: usize, ch: char) -> i32 {
    let max_r = mat.len() as i32;
    let max_c = mat[0].len() as i32;

    let mut ans = 0;

    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nr >= max_r {
                continue;
            }
            if nc < 0 || nc >= max_c {
                continue;
            }
            if mat[nr as usize][nc as usize] == ch {
                ans += 1;
            }
        }
    }

    ans
}

// An open acre will become filled with trees if three or more adjacent acres
// contained trees. Otherwise, nothing happens.
//
// An acre filled with trees will become a lumberyard if three or more adjacent
// acres were lumberyards. Otherwise, nothing happens.
//
// An acre containing a lumberyard will remain a lumberyard if it was adjacent
// to at least one other lumberyard and at least one acre containing trees.
// Otherwise, it becomes open.

fn get_next(cur: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new = Vec::<Vec<char>>::new();

    for r in 0..cur.len() {
        let mut row = Vec::<char>::new();
        for c in 0..cur[0].len() {
            if cur[r][c] == '.' && count_adj(cur, r, c, '|') >= 3 {
                row.push('|');
            } else if cur[r][c] == '|' && count_adj(cur, r, c, '#') >= 3 {
                row.push('#');
            } else if cur[r][c] == '#'
                && (count_adj(cur, r, c, '#') == 0 || count_adj(cur, r, c, '|') == 0)
            {
                row.push('.');
            } else {
                row.push(cur[r][c]);
            }
        }
        new.push(row);
    }

    new
}

fn main() {
    let lines = aoc::read_lines();
    let mut lines: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();

    let mut visited = HashMap::<usize, usize>::new();
    let mut loop_history = Vec::<usize>::new();

    let mut i = 0;
    while i < 1_000_000_000 {
        lines = get_next(&lines);
        let wood = lines
            .iter()
            .flat_map(|r| r.iter())
            .filter(|x| **x == '|')
            .count();
        let lumber = lines
            .iter()
            .flat_map(|r| r.iter())
            .filter(|x| **x == '#')
            .count();
        let ans = wood * lumber;
        if visited.contains_key(&ans) {
            let prev = visited[&ans];
            loop_history.push(i - prev);

            let n = loop_history.len();
            if n > 3
                && loop_history[n - 1] == loop_history[n - 2]
                && loop_history[n - 2] == loop_history[n - 3]
            {
                // loop detected
                let loop_len = loop_history[n - 1];
                let diff = (1_000_000_000 - i) / loop_len;
                let old_i = i;
                i += loop_len * diff;
                println!("skipped from {} to {}", old_i, i);
                loop_history.clear();
                visited.clear();
            }
        }
        visited.insert(ans, i);
        i += 1;
        if i == 1_000_000_000 {
            println!("{}", ans);
        }
    }
}
