import re
from collections import defaultdict

from libs import io

input = io.read_lines()

molecule = input[-1]

gen_map: dict[str, list[str]] = defaultdict(list)
for line in input[:-1]:
    a, b = line.split(" => ")
    gen_map[a].append(b)


ans: set[str] = set()
for a, b_arr in gen_map.items():
    matches = re.findall(a, molecule)
    for match in re.finditer(a, molecule):
        start, end = match.span()
        for b in b_arr:
            ans.add(molecule[:start] + b + molecule[end:])

print(len(ans))
