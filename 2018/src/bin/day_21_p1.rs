use regex::Regex;

#[derive(Debug, PartialEq)]
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

fn run(reg: &[usize; 6], instr: &[usize; 3], op: &Opcode) -> [usize; 6] {
    let mut ans = reg.clone();
    let a = instr[0];
    let b = instr[1];
    let c = instr[2];

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

fn run_loop(ptr_reg: usize, start_reg: usize, lines: &Vec<String>) -> usize {
    let inst_re = Regex::new(r"(\w+) (\d+) (\d+) (\d+)").unwrap();
    let mut reg = [0; 6];
    reg[0] = start_reg;
    let mut ptr = reg[ptr_reg];

    while ptr < lines.len() - 1 {
        let str_instr = &lines[ptr + 1];
        let caps = inst_re.captures(str_instr).unwrap();

        let op_str = &caps[1];
        let op = match op_str {
            "addr" => Opcode::Addr,
            "addi" => Opcode::Addi,
            "mulr" => Opcode::Mulr,
            "muli" => Opcode::Muli,
            "banr" => Opcode::Banr,
            "bani" => Opcode::Bani,
            "borr" => Opcode::Borr,
            "bori" => Opcode::Bori,
            "gtir" => Opcode::Gtir,
            "gtri" => Opcode::Gtri,
            "gtrr" => Opcode::Gtrr,
            "eqir" => Opcode::Eqir,
            "eqrr" => Opcode::Eqrr,
            "eqri" => Opcode::Eqri,
            "setr" => Opcode::Setr,
            "seti" => Opcode::Seti,
            _ => panic!("Invalid opcode: {}", op_str),
        };
        let instr: [usize; 3] = [
            caps[2].parse().unwrap(),
            caps[3].parse().unwrap(),
            caps[4].parse().unwrap(),
        ];
        if op == Opcode::Eqrr {
            return reg[1];
        }
        reg = run(&reg, &instr, &op);
        ptr = reg[ptr_reg];
        ptr += 1;
        reg[ptr_reg] = ptr;
    }

    panic!("unreachable");
}

fn main() {
    let lines = aoc::read_lines();

    let re = Regex::new(r"#ip (\d+)").unwrap();
    let ptr_reg: usize = re.captures(&lines[0]).unwrap()[1].parse().unwrap();

    let ans = run_loop(ptr_reg, 0, &lines);
    println!("{}", ans)
}
