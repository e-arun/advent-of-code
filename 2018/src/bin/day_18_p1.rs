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

    for i in 0..10 {
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
        if i == 9 {
            println!("{}", wood * lumber);
        }
    }
}
