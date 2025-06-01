import re
from typing import Literal

from pydantic import BaseModel

from libs import io


class _Instruction(BaseModel):
    kind: Literal["turn on", "turn off", "toggle"]
    x1: int
    y1: int
    x2: int
    y2: int


def _parse_line(line: str) -> _Instruction:
    kind: Literal["turn on", "turn off", "toggle"]

    if line.startswith("turn on"):
        kind = "turn on"
    elif line.startswith("turn off"):
        kind = "turn off"
    elif line.startswith("toggle"):
        kind = "toggle"
    else:
        assert False

    line = line[len(kind) :].strip()
    match = re.match(r"(\d+),(\d+) through (\d+),(\d+)", line)
    assert match
    groups = match.groups()

    return _Instruction(
        kind=kind,
        x1=int(groups[0]),
        y1=int(groups[1]),
        x2=int(groups[2]),
        y2=int(groups[3]),
    )


input = [_parse_line(line) for line in io.read_lines()]
grid = [[0 for _ in range(1_000)] for _ in range(1_000)]

for instruction in input:
    for x in range(instruction.x1, instruction.x2 + 1):
        for y in range(instruction.y1, instruction.y2 + 1):
            if instruction.kind == "turn on":
                grid[x][y] += 1
            elif instruction.kind == "turn off":
                grid[x][y] = max(0, grid[x][y] - 1)
            else:
                grid[x][y] += 2

ans = sum(sum(row) for row in grid)
print(ans)
