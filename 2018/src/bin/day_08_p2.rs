#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<i32>,
    size: usize,
}

fn parse_node(arr: &[i32]) -> Node {
    let child_ct = arr[0];
    let metadata_ct = arr[1];

    let mut idx = 2;
    let mut children = Vec::new();
    for _ in 0..child_ct {
        let child = parse_node(&arr[idx..]);
        idx += child.size;
        children.push(child);
    }

    let metadata: Vec<i32> = arr[idx..(idx + metadata_ct as usize)]
        .iter()
        .map(|x| *x)
        .collect();
    idx += metadata_ct as usize;

    Node {
        children,
        metadata,
        size: idx,
    }
}

fn get_ans(node: &Node) -> i32 {
    if node.children.len() == 0 {
        return node.metadata.iter().sum();
    }

    let mut ans = 0;
    for idx in node.metadata.iter() {
        if *idx == 0 {
            continue;
        }
        if let Some(child) = node.children.get((*idx - 1) as usize) {
            ans += get_ans(child);
        }
    }

    ans
}

fn main() {
    let arr: Vec<i32> = aoc::read_input()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let root = parse_node(&arr);
    let ans = get_ans(&root);
    println!("{ans}");
}
