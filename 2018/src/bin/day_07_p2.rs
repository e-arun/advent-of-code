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
    let mut max_time = 0;
    let steps_len = steps.len();
    let mut queue: BinaryHeap<_> = steps
        .into_iter()
        .filter(|x| !deps.contains_key(x))
        .map(|x| Reverse(x))
        .collect();
    let mut workers: BinaryHeap<(Reverse<i32>, Option<&str>)> =
        (0..5).map(|_| (Reverse(0), None)).collect();
    let mut free_workers = 0;

    while ans.len() < steps_len {
        let worker = workers.pop().unwrap();
        if let Some(done_step) = worker.1 {
            max_time = worker.0.0;
            ans.push(done_step);

            if !deps_rev.contains_key(done_step) {
                continue;
            }
            for dep in deps_rev[done_step].iter() {
                deps.get_mut(dep).unwrap().remove(done_step);
                if deps[dep].len() == 0 {
                    queue.push(Reverse(dep));
                }
            }

            for _ in 0..free_workers {
                workers.push((worker.0, None));
            }
            free_workers = 0;
        }

        if let None = queue.peek() {
            free_workers += 1;
            continue;
        }

        let step = queue.pop().unwrap().0;
        let step_time = step.bytes().next().unwrap() - 'A' as u8 + 61;
        workers.push((Reverse(worker.0.0 + step_time as i32), Some(step)));
    }

    println!("{max_time}");
}
