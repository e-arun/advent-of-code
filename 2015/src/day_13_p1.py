import itertools

from libs import io


def _parse_line(line: str) -> tuple[str, str, int]:
    words = line.split()

    src = words[0]
    dst = words[-1].rstrip(".")
    val = int(words[3])
    gain_or_loose = words[2]

    if gain_or_loose == "lose":
        val = -val

    return (src, dst, val)


def _get_total_happiness(
    arrangement: list[str], happiness_map: dict[tuple[str, str], int]
) -> int:
    happiness = 0
    arrangement = [arrangement[-1], *arrangement, arrangement[0]]

    for left, middle, right in zip(arrangement, arrangement[1:], arrangement[2:]):
        happiness += happiness_map[(middle, right)]
        happiness += happiness_map[(middle, left)]

    return happiness


lines = [_parse_line(line) for line in io.read_lines()]
happiness_map = {(src, dst): val for src, dst, val in lines}
all_names = {src for src, _ in happiness_map}

ans = max(
    _get_total_happiness(list(arrangement), happiness_map)
    for arrangement in itertools.permutations(all_names)
)
print(ans)
