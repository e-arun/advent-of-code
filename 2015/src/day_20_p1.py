from libs import io

MAX_HOUSES = 1_000_000

target_presents = int(io.read_all())
presents_arr = [0 for _ in range(MAX_HOUSES)]

for i in range(1, MAX_HOUSES):
    idx = i
    present_count = i * 10

    while idx < MAX_HOUSES:
        presents_arr[idx] += present_count
        idx += i

ans = next(i for i, presents in enumerate(presents_arr) if presents >= target_presents)
print(ans)
