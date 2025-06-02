import itertools

from libs import io

containers = [int(line) for line in io.read_lines()]
target = 150

ans = 0
for i in range(1, len(containers) + 1):
    for comb in itertools.combinations(containers, i):
        if sum(comb) == target:
            ans += 1

print(ans)
