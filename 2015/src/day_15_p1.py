from pydantic import BaseModel

from libs import io


class _Ingredient(BaseModel):
    capacity: int
    durability: int
    flavor: int
    texture: int
    calories: int

    def __add__(self, other: "_Ingredient") -> "_Ingredient":
        if not isinstance(other, _Ingredient):
            raise ValueError

        return _Ingredient(
            capacity=self.capacity + other.capacity,
            durability=self.durability + other.durability,
            flavor=self.flavor + other.flavor,
            texture=self.texture + other.texture,
            calories=self.calories + other.calories,
        )

    def __mul__(self, other: int) -> "_Ingredient":
        if not isinstance(other, int):
            raise ValueError

        return _Ingredient(
            capacity=self.capacity * other,
            durability=self.durability * other,
            flavor=self.flavor * other,
            texture=self.texture * other,
            calories=self.calories * other,
        )

    def get_score(self) -> int:
        if (
            self.capacity < 0
            or self.durability < 0
            or self.flavor < 0
            or self.texture < 0
        ):
            return 0
        return self.capacity * self.durability * self.flavor * self.texture


def _parse_line(line: str) -> _Ingredient:
    words = line.split()
    words = [word.removesuffix(",") for word in words]
    return _Ingredient(
        capacity=int(words[2]),
        durability=int(words[4]),
        flavor=int(words[6]),
        texture=int(words[8]),
        calories=int(words[10]),
    )


input = [_parse_line(line) for line in io.read_lines()]
MAX_TSP = 100


def _traverse(idx: int, tsp: int, ingredients: _Ingredient) -> int:
    if tsp > MAX_TSP:
        return 0

    if idx == len(input) - 1:
        cur_tsp = MAX_TSP - tsp
        ingredients = ingredients + input[idx] * cur_tsp
        return ingredients.get_score()

    cur_ingredient = input[idx]
    return max(
        _traverse(idx + 1, tsp + cur_tsp, ingredients + cur_ingredient * cur_tsp)
        for cur_tsp in range(0, MAX_TSP - tsp + 1)
    )


no_ingredient = _Ingredient(capacity=0, durability=0, flavor=0, texture=0, calories=0)
ans = _traverse(0, 0, no_ingredient)
print(ans)
