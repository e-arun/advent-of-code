from copy import deepcopy

from libs import io

_MAT_SIZE = 100


def _get_adjacent_on(mat: list[list[str]], x: int, y: int) -> int:
    on_lights = 0

    for x_diff in (-1, 0, 1):
        for y_diff in (-1, 0, 1):
            if x_diff == 0 and y_diff == 0:
                continue
            new_x = x + x_diff
            new_y = y + y_diff
            if not 0 <= new_x < _MAT_SIZE:
                continue
            if not 0 <= new_y < _MAT_SIZE:
                continue
            on_lights += int(mat[new_x][new_y] == "#")

    return on_lights


def _set_corner_lights(mat: list[list[str]]) -> None:
    mat[0][0] = "#"
    mat[-1][0] = "#"
    mat[0][-1] = "#"
    mat[-1][-1] = "#"


def _get_next_mat(mat: list[list[str]]) -> list[list[str]]:
    new_mat = deepcopy(mat)
    for x in range(_MAT_SIZE):
        for y in range(_MAT_SIZE):
            adj_on = _get_adjacent_on(mat, x, y)
            if mat[x][y] == "#" and adj_on not in (2, 3):
                new_mat[x][y] = "."
            elif mat[x][y] == "." and adj_on == 3:
                new_mat[x][y] = "#"
    return new_mat


mat = [list(line) for line in io.read_lines()]
_set_corner_lights(mat)

for _ in range(100):
    mat = _get_next_mat(mat)
    _set_corner_lights(mat)

ans = sum(row.count("#") for row in mat)
print(ans)
