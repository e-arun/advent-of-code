from pydantic import BaseModel

from libs import io


class _Deer(BaseModel):
    speed: int
    active: int
    rest: int


def _parse_line(line: str) -> _Deer:
    words = line.split()
    return _Deer(speed=int(words[3]), active=int(words[6]), rest=int(words[13]))


def _dist_traveled(deer: _Deer, time: int) -> int:
    dist = 0

    while time > 0:
        dist += deer.speed * min(time, deer.active)
        time -= deer.active
        time -= deer.rest

    return dist


deer = [_parse_line(line) for line in io.read_lines()]
ans = max(_dist_traveled(x, 2503) for x in deer)
print(ans)
