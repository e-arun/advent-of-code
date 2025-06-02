from pydantic import BaseModel, ConfigDict

from libs import io


class _Deer(BaseModel):
    speed: int
    active: int
    rest: int
    model_config = ConfigDict(frozen=True)


class _DeerState(BaseModel):
    score: int
    dist: int
    is_active: bool
    remaining_time: int


def _parse_line(line: str) -> _Deer:
    words = line.split()
    return _Deer(speed=int(words[3]), active=int(words[6]), rest=int(words[13]))


def _sim_scores(deer_arr: list[_Deer], time: int) -> dict[_Deer, int]:
    state_map = {
        deer: _DeerState(dist=0, score=0, is_active=True, remaining_time=deer.active)
        for deer in deer_arr
    }

    for _ in range(time):
        for deer, state in state_map.items():
            if state.is_active:
                state.dist += deer.speed

            state.remaining_time -= 1
            if state.remaining_time == 0:
                if state.is_active:
                    state.is_active = False
                    state.remaining_time = deer.rest
                else:
                    state.is_active = True
                    state.remaining_time = deer.active

        max_dist = max(state.dist for state in state_map.values())
        for deer, state in state_map.items():
            if state.dist == max_dist:
                state.score += 1

    return {deer: state_map[deer].score for deer in deer_arr}


deer = [_parse_line(line) for line in io.read_lines()]
deer_scores = _sim_scores(deer, 2503)
ans = max(deer_scores.values())
print(ans)
