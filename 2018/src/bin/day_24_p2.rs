use std::collections::HashSet;

use regex::Regex;

#[derive(Debug, Clone)]
struct Group {
    units: i32,
    hp: i32,
    weak: HashSet<String>,
    immune: HashSet<String>,
    attack: String,
    dmg: i32,
    initiative: i32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum GroupType {
    Infection,
    Immune,
}

fn parse_group(line: &str) -> Group {
    let re = Regex::new(
        r"(\d+) units each with (\d+) hit points(:? \(([^)]+)\))? with an attack that does (\d+) (\w+) damage at initiative (\d+)",
    ).unwrap();
    let caps = re.captures(line).unwrap();

    let mut weak = HashSet::new();
    let mut immune = HashSet::new();
    if let Some(info) = caps.get(4) {
        for val in info.as_str().split("; ") {
            if val.starts_with("weak to ") {
                for item in val[8..].split(", ") {
                    weak.insert(item.to_string());
                }
            } else {
                for item in val[10..].split(", ") {
                    immune.insert(item.to_string());
                }
            }
        }
    }

    Group {
        units: caps[1].parse().unwrap(),
        hp: caps[2].parse().unwrap(),
        weak: weak,
        immune: immune,
        attack: caps[6].to_string(),
        dmg: caps[5].parse().unwrap(),
        initiative: caps[7].parse().unwrap(),
    }
}

fn get_dmg(attacker: &Group, defender: &Group) -> i32 {
    let power = attacker.units * attacker.dmg;
    if defender.immune.contains(&attacker.attack) {
        return 0;
    }
    if defender.weak.contains(&attacker.attack) {
        return power * 2;
    }
    return power;
}

fn pick_target(group: &Group, targets: &Vec<Group>, picked: &HashSet<usize>) -> Option<usize> {
    let mut arr = Vec::new();
    for (i, target) in targets.iter().enumerate() {
        if picked.contains(&i) {
            continue;
        }
        let dmg = get_dmg(group, target);
        if dmg != 0 {
            arr.push((dmg, target.units * target.dmg, target.initiative, i));
        }
    }
    arr.iter().max().and_then(|x| Some(x.3))
}

fn sim_iter(infection_groups: &mut Vec<Group>, immune_groups: &mut Vec<Group>) -> bool {
    let mut select_order = Vec::new();
    for (i, group) in infection_groups.iter().enumerate() {
        select_order.push((
            group.units * group.dmg,
            group.initiative,
            GroupType::Infection,
            i,
        ));
    }
    for (i, group) in immune_groups.iter().enumerate() {
        select_order.push((
            group.units * group.dmg,
            group.initiative,
            GroupType::Immune,
            i,
        ));
    }
    select_order.sort();
    select_order.reverse();

    let mut picked_infection = HashSet::new();
    let mut picked_immune = HashSet::new();
    let mut attack_order = Vec::new();
    for (_, _, group_type, idx) in select_order {
        if group_type == GroupType::Infection {
            let group = &infection_groups[idx];
            let target = pick_target(group, immune_groups, &picked_immune);
            if let Some(target_idx) = target {
                picked_immune.insert(target_idx);
                attack_order.push((group.initiative, GroupType::Infection, idx, target_idx));
            }
        } else {
            let group = &immune_groups[idx];
            let target = pick_target(group, infection_groups, &picked_infection);
            if let Some(target_idx) = target {
                picked_infection.insert(target_idx);
                attack_order.push((group.initiative, GroupType::Immune, idx, target_idx));
            }
        }
    }

    attack_order.sort();
    attack_order.reverse();
    let mut kill_count = 0;

    for (_, group_type, idx, target_idx) in attack_order {
        if group_type == GroupType::Infection {
            let group = &infection_groups[idx];
            let target = &mut immune_groups[target_idx];
            let dmg = get_dmg(group, target);
            if dmg == 0 {
                continue;
            }
            let units_loss = std::cmp::min(target.units, dmg / target.hp);
            target.units -= units_loss;
            kill_count += units_loss;
        } else {
            let group = &immune_groups[idx];
            let target = &mut infection_groups[target_idx];
            let dmg = get_dmg(group, target);
            if dmg == 0 {
                continue;
            }
            let units_loss = std::cmp::min(target.units, dmg / target.hp);
            target.units -= units_loss;
            kill_count += units_loss;
        }
    }

    infection_groups.retain(|x| x.units != 0);
    immune_groups.retain(|x| x.units != 0);
    return kill_count > 0;
}

fn win_units(boost: i32, mut immune_groups: Vec<Group>, mut infection_groups: Vec<Group>) -> i32 {
    for group in immune_groups.iter_mut() {
        group.dmg += boost;
    }
    while !infection_groups.is_empty() && !immune_groups.is_empty() {
        let has_kills = sim_iter(&mut infection_groups, &mut immune_groups);
        if !has_kills {
            return 0;
        }
    }

    immune_groups.iter().map(|x| x.units).sum()
}

fn main() {
    let lines = aoc::read_lines();

    let mut immune_groups = Vec::new();
    let mut infection_groups = Vec::new();

    let mut is_infection = false;
    for line in lines {
        if line == "Immune System:" || line == "" {
            continue;
        }
        if line == "Infection:" {
            is_infection = true;
            continue;
        }
        if is_infection {
            infection_groups.push(parse_group(&line));
        } else {
            immune_groups.push(parse_group(&line));
        }
    }

    // Ideally binary search can be used for a more optimal solution, however
    // based on the input it seemed like the value of boost wasn't going to be
    // much larger than 1000 and brute force would be enough
    for boost in 1.. {
        let ans = win_units(boost, immune_groups.clone(), infection_groups.clone());
        if ans > 0 {
            println!("{}", ans);
            break;
        }
    }
}
