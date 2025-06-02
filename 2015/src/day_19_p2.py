from libs import io

input = io.read_lines()

molecule = input[-1]

reduce_arr: list[tuple[str, str]] = []
for line in input[:-1]:
    a, b = line.split(" => ")
    reduce_arr.append((a, b))
reduce_arr.sort(key=lambda x: len(x[1]), reverse=True)

# Greedily try to replace the largest substring.
# I don't think this produces the optimal solution for all inputs, but the input
# is probably intentionally constructed in a way that allows for this.

ans = 0
while molecule != "e":
    for a, b in reduce_arr:
        if b in molecule:
            molecule = molecule.replace(b, a, 1)
            ans += 1


print(ans)
