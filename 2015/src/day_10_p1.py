import itertools

from libs import io


def _look_and_say(xs: str) -> str:
    say_arr = []
    for key, group in itertools.groupby(xs):
        group_len = len(list(group))
        say_arr.append(str(group_len))
        say_arr.append(key)
    return "".join(say_arr)


input = io.read_all()

ans = input
for _ in range(40):
    ans = _look_and_say(ans)
print(len(ans))
