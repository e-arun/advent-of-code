from libs import io


def _is_nice(xs: str) -> bool:
    vowels = [x for x in xs if x in "aeoiu"]
    if len(vowels) < 3:
        return False

    twice_in_a_row = [a == b for a, b in zip(xs, xs[1:])]
    if not any(twice_in_a_row):
        return False

    substr_arr = ["ab", "cd", "pq", "xy"]
    if any(substr in xs for substr in substr_arr):
        return False

    return True


input = io.read_lines()
ans = sum(_is_nice(x) for x in input)
print(ans)
