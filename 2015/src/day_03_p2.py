from libs import io

input = io.read_all()

santa_pos = [0, 0]
robot_pos = [0, 0]
visited: set[tuple[int, ...]] = {(0, 0)}

for i, dir in enumerate(input):

    if i % 2 == 0:
        cur_pos = santa_pos
    else:
        cur_pos = robot_pos

    match dir:
        case "^":
            cur_pos[1] += 1
        case ">":
            cur_pos[0] += 1
        case "v":
            cur_pos[1] -= 1
        case "<":
            cur_pos[0] -= 1
    visited.add(tuple(cur_pos))


print(len(visited))
