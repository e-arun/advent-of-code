use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
enum Msg {
    Guard(i32),
    Wake,
    Sleep,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
struct Timestamp {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    min: i32,
}

#[derive(Debug)]
struct Record {
    ts: Timestamp,
    msg: Msg,
}

fn parse_line(s: &str) -> Record {
    let re = Regex::new(r"\[(\d+)\-(\d+)\-(\d+) (\d+):(\d+)\] (.*)").unwrap();
    let caps = re.captures(&s).unwrap();

    let ts = Timestamp {
        year: caps[1].parse().unwrap(),
        month: caps[2].parse().unwrap(),
        day: caps[3].parse().unwrap(),
        hour: caps[4].parse().unwrap(),
        min: caps[5].parse().unwrap(),
    };

    let msg_str = &caps[6];
    if msg_str.starts_with("Guard") {
        let re = Regex::new(r"Guard #(\d+)").unwrap();
        let guard_id: i32 = re.captures(msg_str).unwrap()[1].parse().unwrap();
        return Record {
            ts,
            msg: Msg::Guard(guard_id),
        };
    }
    if msg_str == "wakes up" {
        return Record { ts, msg: Msg::Wake };
    }
    if msg_str == "falls asleep" {
        return Record {
            ts,
            msg: Msg::Sleep,
        };
    }

    panic!("{s}");
}

fn update_sleep(guard_id: i32, last_sleep: i32, cur_min: i32, map: &mut HashMap<(i32, i32), i32>) {
    if last_sleep == -1 || guard_id == -1 {
        return;
    }
    for i in last_sleep..cur_min {
        let ctr = map.entry((guard_id, i)).or_insert(0);
        *ctr += 1;
    }
}

fn main() {
    let mut records: Vec<_> = aoc::read_lines().iter().map(|x| parse_line(x)).collect();
    records.sort_by(|a, b| a.ts.cmp(&b.ts));

    let mut map = HashMap::<(i32, i32), i32>::new();
    let mut guard_id = -1;
    let mut last_sleep = -1;

    for record in &records {
        match record.msg {
            Msg::Guard(gid) => {
                update_sleep(guard_id, last_sleep, 60, &mut map);
                guard_id = gid;
                last_sleep = -1;
            }
            Msg::Wake => {
                update_sleep(guard_id, last_sleep, record.ts.min, &mut map);
                last_sleep = -1;
            }
            Msg::Sleep => {
                last_sleep = record.ts.min;
            }
        }
    }

    let mut guard_sleep = HashMap::<i32, i32>::new();
    for ((guard_id, _), ct) in &map {
        let duration = guard_sleep.entry(*guard_id).or_insert(0);
        *duration += ct;
    }

    let max_guard_id = guard_sleep.iter().max_by_key(|x| *x.1).unwrap().0;
    let max_min = map
        .iter()
        .filter(|x| x.0.0 == *max_guard_id)
        .max_by_key(|x| x.1)
        .unwrap()
        .0
        .1;
    let ans = max_guard_id * max_min;

    println!("{ans}");
}
