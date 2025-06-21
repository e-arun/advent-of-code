use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

fn main() {
    let lines = aoc::read_lines();

    let mut deps = HashMap::<&str, HashSet<&str>>::new();
    let mut deps_rev = HashMap::<&str, HashSet<&str>>::new();
    let mut steps = HashSet::<&str>::new();

    for line in lines.iter() {
        let arr: Vec<_> = line.split_whitespace().collect();

        let entry = deps.entry(arr[7]).or_insert(HashSet::new());
        entry.insert(arr[1]);

        let entry = deps_rev.entry(arr[1]).or_insert(HashSet::new());
        entry.insert(arr[7]);

        steps.insert(arr[1]);
        steps.insert(arr[7]);
    }

    let mut ans = Vec::<&str>::new();
    let mut cur: BinaryHeap<_> = steps
        .into_iter()
        .filter(|x| !deps.contains_key(x))
        .map(|x| Reverse(x))
        .collect();

    while cur.len() != 0 {
        let step = cur.pop().unwrap().0;
        ans.push(step);

        if !deps_rev.contains_key(step) {
            continue;
        }
        for dep in deps_rev[step].iter() {
            deps.get_mut(dep).unwrap().remove(step);
            if deps[dep].len() == 0 {
                cur.push(Reverse(dep));
            }
        }
    }

    let ans = ans.join("");
    println!("{ans}")
}
