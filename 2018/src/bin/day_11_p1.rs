fn get_power(x: i32, y: i32, grid: i32) -> i32 {
    let mut power = x + 10;
    power *= y;
    power += grid;
    power *= x + 10;
    power = (power / 100) % 10;
    power -= 5;
    power
}

fn main() {
    let input: i32 = aoc::read_input().parse().unwrap();

    let mut ans = -10;
    let mut pos = (0, 0);

    for x in 1..=298 {
        for y in 1..=298 {
            let mut power = 0;
            for dx in 0..=2 {
                for dy in 0..=2 {
                    power += get_power(x + dx, y + dy, input);
                }
            }
            if power > ans {
                ans = power;
                pos = (x, y);
            }
        }
    }

    println!("{},{}", pos.0, pos.1);
}
