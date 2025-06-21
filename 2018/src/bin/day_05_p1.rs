fn main() {
    let input = aoc::read_input();

    let mut arr: Vec<_> = input.chars().collect();
    loop {
        let mut flag = true;
        let mut new_arr = Vec::<char>::new();

        let mut i = 0;
        while i < arr.len() {
            if i == arr.len() - 1 {
                new_arr.push(arr[i]);
                i += 1;
                continue;
            }

            if arr[i] != arr[i + 1]
                && arr[i].to_ascii_lowercase() == arr[i + 1].to_ascii_lowercase()
            {
                flag = false;
                i += 2;
                continue;
            }
            new_arr.push(arr[i]);
            i += 1;
            continue;
        }

        if flag {
            break;
        }
        arr = new_arr;
    }

    let ans = arr.len();
    println!("{ans}");
}
