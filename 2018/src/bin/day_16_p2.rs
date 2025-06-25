use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug)]
struct Example {
    before: [usize; 4],
    instr: [usize; 4],
    after: [usize; 4],
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
    Setr,
    Seti,
}

fn run(reg: &[usize; 4], instr: &[usize; 4], op: &Opcode) -> [usize; 4] {
    let mut ans = reg.clone();
    let a = instr[1];
    let b = instr[2];
    let c = instr[3];

    match op {
        Opcode::Addr => {
            ans[c] = ans[a] + ans[b];
        }
        Opcode::Addi => {
            ans[c] = ans[a] + b;
        }
        Opcode::Mulr => {
            ans[c] = ans[a] * ans[b];
        }
        Opcode::Muli => {
            ans[c] = ans[a] * b;
        }
        Opcode::Banr => {
            ans[c] = ans[a] & ans[b];
        }
        Opcode::Bani => {
            ans[c] = ans[a] & b;
        }
        Opcode::Borr => {
            ans[c] = ans[a] | ans[b];
        }
        Opcode::Bori => {
            ans[c] = ans[a] | b;
        }
        Opcode::Gtir => {
            ans[c] = if a > ans[b] { 1 } else { 0 };
        }
        Opcode::Gtri => {
            ans[c] = if ans[a] > b { 1 } else { 0 };
        }
        Opcode::Gtrr => {
            ans[c] = if ans[a] > ans[b] { 1 } else { 0 };
        }
        Opcode::Eqir => {
            ans[c] = if a == ans[b] { 1 } else { 0 };
        }
        Opcode::Eqri => {
            ans[c] = if ans[a] == b { 1 } else { 0 };
        }
        Opcode::Eqrr => {
            ans[c] = if ans[a] == ans[b] { 1 } else { 0 };
        }
        Opcode::Setr => {
            ans[c] = ans[a];
        }
        Opcode::Seti => {
            ans[c] = a;
        }
    }

    ans
}

fn main() {
    let lines = aoc::read_lines();
    let mut examples = Vec::<Example>::new();

    for x in &lines.iter().chunks(4) {
        let arr: Vec<_> = x.collect();
        if !arr[0].starts_with("Before:") {
            break;
        }

        let before: Vec<usize> = arr[0][9..(arr[0].len() - 1)]
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();
        let after: Vec<usize> = arr[2][9..(arr[0].len() - 1)]
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();
        let instr: Vec<usize> = arr[1].split(" ").map(|x| x.parse().unwrap()).collect();

        let example = Example {
            before: [before[0], before[1], before[2], before[3]],
            instr: [instr[0], instr[1], instr[2], instr[3]],
            after: [after[0], after[1], after[2], after[3]],
        };
        examples.push(example);
    }

    let all_codes = [
        Opcode::Addr,
        Opcode::Addi,
        Opcode::Mulr,
        Opcode::Muli,
        Opcode::Banr,
        Opcode::Bani,
        Opcode::Borr,
        Opcode::Bori,
        Opcode::Gtir,
        Opcode::Gtri,
        Opcode::Gtrr,
        Opcode::Eqir,
        Opcode::Eqri,
        Opcode::Eqrr,
        Opcode::Setr,
        Opcode::Seti,
    ];

    let mut code_map = HashMap::<usize, HashSet<&Opcode>>::new();
    for example in examples.iter() {
        let code_id = example.instr[0];
        if !code_map.contains_key(&code_id) {
            code_map.insert(code_id, all_codes.iter().collect());
        }
    }

    for example in examples.iter() {
        let code_set = code_map.get_mut(&example.instr[0]).unwrap();
        for code in all_codes.iter() {
            if run(&example.before, &example.instr, code) != example.after {
                code_set.remove(code);
            }
        }
    }

    loop {
        let mut flag = false;
        let mut taken = HashSet::new();

        for (_, v) in code_map.iter() {
            if v.len() == 1 {
                taken.insert(*v.iter().next().unwrap());
            }
        }

        for (_, v) in code_map.iter_mut() {
            if v.len() == 1 {
                continue;
            }
            for x in taken.iter() {
                v.remove(x);
            }
            if v.len() == 1 {
                flag = true;
            }
        }

        if !flag {
            break;
        }
    }

    let mut reg = [0 as usize; 4];
    for line in lines[(examples.len() * 4)..].iter() {
        if line == "" {
            continue;
        }
        let instr: Vec<usize> = line.split(" ").map(|x| x.parse().unwrap()).collect();
        let instr = [instr[0], instr[1], instr[2], instr[3]];
        let code = *code_map.get(&instr[0]).unwrap().iter().next().unwrap();
        reg = run(&reg, &instr, code);
    }
    println!("{}", reg[0]);
}
