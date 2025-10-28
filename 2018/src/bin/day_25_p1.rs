fn get_root(i: usize, parents: &mut Vec<usize>) -> usize {
    if parents[i] != i {
        parents[i] = get_root(parents[i], parents);
    }
    parents[i]
}

fn union(i: usize, j: usize, parents: &mut Vec<usize>) {
    let root_i = get_root(i, parents);
    let root_j = get_root(j, parents);
    parents[root_i] = root_j;
}

fn main() {
    let lines = aoc::read_lines();
    let points = lines
        .iter()
        .map(|x| {
            x.split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut parents = vec![0; points.len()];
    for i in 0..parents.len() {
        parents[i] = i;
    }

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let pi = &points[i];
            let pj = &points[j];
            let dist: i32 = pi.iter().zip(pj).map(|(a, b)| (a - b).abs()).sum();
            if dist <= 3 {
                union(i, j, &mut parents);
            }
        }
    }

    let ans = parents.iter().enumerate().filter(|(i, x)| i == *x).count();
    println!("{}", ans);
}
