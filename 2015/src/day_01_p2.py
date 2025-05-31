from libs import io

input = io.read_all()

cur_floor = 0
cur_idx = 0

for cur_idx, ch in enumerate(input):
    match ch:
        case "(":
            cur_floor += 1
        case ")":
            cur_floor -= 1
    if cur_floor < 0:
        break

print(cur_idx + 1)
