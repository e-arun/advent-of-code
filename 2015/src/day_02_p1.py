import re

from libs import io


def _parse_line(line: str) -> tuple[int, int, int]:
    match = re.match(r"(\d+)x(\d+)x(\d+)", line)
    assert match
    a, b, c = match.groups()
    return (int(a), int(b), int(c))


input = [_parse_line(line) for line in io.read_lines()]

ans = 0
for a, b, c in input:
    areas = (a * b, b * c, c * a)
    ans += 2 * sum(areas) + min(areas)

print(ans)
