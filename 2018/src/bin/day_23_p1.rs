use regex::Regex;

struct Bot {
    x: i32,
    y: i32,
    z: i32,
    r: i32,
}

fn count_in_range(main_bot: &Bot, bots: &Vec<Bot>) -> i32 {
    let mut ans = 0;

    for bot in bots {
        let dist =
            (main_bot.x - bot.x).abs() + (main_bot.y - bot.y).abs() + (main_bot.z - bot.z).abs();
        if dist <= main_bot.r {
            ans += 1;
        }
    }

    ans
}

fn main() {
    let lines = aoc::read_lines();
    let re = Regex::new(r"pos=<([-]?\d+),([-]?\d+),([-]?\d+)>, r=(\d+)").unwrap();

    let mut bots = Vec::new();
    for line in lines {
        let caps = re.captures(&line).unwrap();
        bots.push(Bot {
            x: caps[1].parse().unwrap(),
            y: caps[2].parse().unwrap(),
            z: caps[3].parse().unwrap(),
            r: caps[4].parse().unwrap(),
        });
    }

    let max_r = bots.iter().map(|x| x.r).max().unwrap();
    let mut ans = 0;

    for bot in bots.iter() {
        if bot.r != max_r {
            continue;
        }
        ans = std::cmp::max(ans, count_in_range(&bot, &bots));
    }
    println!("{}", ans);
}
