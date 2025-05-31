from libs import io

input = io.read_all()

cur_pos = (0, 0)
visited: set[tuple[int, int]] = {cur_pos}

for dir in input:
    x, y = cur_pos
    match dir:
        case "^":
            cur_pos = (x, y + 1)
        case ">":
            cur_pos = (x + 1, y)
        case "v":
            cur_pos = (x, y - 1)
        case "<":
            cur_pos = (x - 1, y)
    visited.add(cur_pos)

print(len(visited))
