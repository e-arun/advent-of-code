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
    let mut pos = (0, 0, 0);

    // Ideal solution would be to use a 2D prefix-sum to speed this up,
    // however, it seems like the solution is usually found in smaller values of 'd'
    // but not sure if this will work for all inputs.
    for d in 1..50 {
        for x in 1..=(301 - d) {
            for y in 1..=(301 - d) {
                let mut power = 0;
                for dx in 0..=(d - 1) {
                    for dy in 0..=(d - 1) {
                        power += get_power(x + dx, y + dy, input);
                    }
                }
                if power > ans {
                    ans = power;
                    pos = (x, y, d);
                }
            }
        }
    }

    println!("{},{},{}", pos.0, pos.1, pos.2);
}
