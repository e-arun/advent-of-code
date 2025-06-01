import itertools

from libs import io

dist_map: dict[tuple[str, str], int] = {}

for line in io.read_lines():
    words = line.split()
    src = words[0]
    dst = words[2]
    dist = int(words[4])

    dist_map[(src, dst)] = dist
    dist_map[(dst, src)] = dist

all_locations = set(k[0] for k in dist_map)


def _get_dist(perm: tuple[str, ...]) -> int:
    return sum(dist_map[(src, dst)] for src, dst in itertools.pairwise(perm))


ans = min(_get_dist(perm) for perm in itertools.permutations(all_locations))
print(ans)
