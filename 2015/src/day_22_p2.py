import functools

from pydantic import BaseModel

from libs import io


class GameState(BaseModel):
    player_hp: int
    boss_hp: int
    boss_damage: int
    mana: int
    shield_duration: int
    poison_duration: int
    recharge_duration: int
    is_player_turn: bool

    def __hash__(self) -> int:
        return hash(self.model_dump_json())


_VISITED = set()


@functools.cache
def get_min_mana(state: GameState) -> int:
    state_hash = hash(state)
    if state_hash in _VISITED:
        return float("inf")  # type: ignore
    _VISITED.add(state_hash)

    if state.is_player_turn:
        state.player_hp -= 1
        if state.player_hp <= 0:
            return float("inf")  # type: ignore

    player_armor = 7 if state.shield_duration else 0

    if state.shield_duration:
        state.shield_duration -= 1
    if state.poison_duration:
        state.poison_duration -= 1
        state.boss_hp -= 3
        if state.boss_hp <= 0:
            return 0
    if state.recharge_duration:
        state.recharge_duration -= 1
        state.mana += 101

    if not state.is_player_turn:
        state.player_hp -= max(1, state.boss_damage - player_armor)
        if state.player_hp <= 0:
            return float("inf")  # type: ignore

        state.is_player_turn = True
        return get_min_mana(state)

    state.is_player_turn = False
    choices = []

    if state.mana >= 53:
        tmp_state = state.model_copy()
        tmp_state.mana -= 53
        tmp_state.boss_hp -= 4
        if tmp_state.boss_hp <= 0:
            choices.append(53)
        else:
            choices.append(53 + get_min_mana(tmp_state))

    if state.mana >= 73:
        tmp_state = state.model_copy()
        tmp_state.mana -= 73
        tmp_state.boss_hp -= 2
        tmp_state.player_hp += 2
        if tmp_state.boss_hp <= 0:
            choices.append(73)
        else:
            choices.append(73 + get_min_mana(tmp_state))

    if state.mana >= 113 and state.shield_duration == 0:
        tmp_state = state.model_copy()
        tmp_state.mana -= 113
        tmp_state.shield_duration = 6
        choices.append(113 + get_min_mana(tmp_state))

    if state.mana >= 173 and state.poison_duration == 0:
        tmp_state = state.model_copy()
        tmp_state.mana -= 173
        tmp_state.poison_duration = 6
        choices.append(173 + get_min_mana(tmp_state))

    if state.mana >= 229 and state.recharge_duration == 0:
        tmp_state = state.model_copy()
        tmp_state.mana -= 229
        tmp_state.recharge_duration = 5
        choices.append(229 + get_min_mana(tmp_state))

    if len(choices) == 0:
        return float("inf")  # type: ignore
    return min(choices)


input = io.read_lines()
state = GameState(
    player_hp=50,
    boss_hp=int(input[0].split()[-1]),
    boss_damage=int(input[1].split()[-1]),
    mana=500,
    shield_duration=0,
    poison_duration=0,
    recharge_duration=0,
    is_player_turn=True,
)
ans = get_min_mana(state)
print(ans)
